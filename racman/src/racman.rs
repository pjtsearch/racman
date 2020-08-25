use quest::yesno;

use crate::transaction::{Transaction};
use crate::transaction::install::{InstallTransaction};
use crate::transaction::upgrade::{UpgradeTransaction};
use crate::transaction::remove::{RemoveTransaction};

use std::rc::Rc;
use alpm::{Alpm,TransFlag,SigLevel};

use std::io;
use std::io::Write;

pub struct Racman {
    pub alpm:Alpm,
    transactions:Vec<Rc<dyn Transaction>>
}

impl Racman {
    pub fn new<'a>()->Result<Racman,alpm::Error>{     
        match Alpm::new("/","/var/lib/pacman") {
            Ok(alpm)=>{
                Ok(Racman {
                    alpm,
                    transactions:vec![]
                })
            },
            Err(err)=>Err(err)
        }
    }
    pub fn register_syncdb(&mut self,repo_name:&str,server:&str){
        let syncdb = self.alpm.register_syncdb_mut(repo_name, SigLevel::NONE).unwrap();
        syncdb.add_server(server)
            .unwrap();
    }
    pub fn add_install(&mut self,repo_name:&str,name:&str){
        self.transactions.push(Rc::new(InstallTransaction{repo_name:repo_name.to_owned().clone(),name:name.to_owned().clone()}));
    }
    pub fn add_upgrade(&mut self){
        self.transactions.push(Rc::new(UpgradeTransaction{}));
    }
    pub fn add_remove(&mut self,name:&str){
        self.transactions.push(Rc::new(RemoveTransaction{name:name.to_owned()}));
    }
    pub fn commit_transaction(&mut self){
        let add_transactions = |transactions:&Vec<Rc<dyn Transaction>>,alpm:&mut Alpm| {
            transactions.iter().for_each(|transaction|{
                transaction.add(alpm);
            });
        };
        self.alpm.trans_init(TransFlag::NONE).expect("couldn't init transaction");
        add_transactions(&self.transactions,&mut self.alpm);
        println!("Transaction Summary:");
        println!("To be added:");
        self.alpm.trans_add().into_iter().for_each(|pkg|{
            print!("{}-{} ",pkg.name(),pkg.version())
        });
        println!();
        println!("To be removed:");
        self.alpm.trans_remove().into_iter().for_each(|pkg|{
            print!("{}-{} ",pkg.name(),pkg.version())
        });
        println!();
        print!("Commit transaction? [y/N]:");
        io::stdout().flush().unwrap();
        let question = yesno(false);
        if let Ok(opt) = question {
            if let Some(choice) = opt{
                if choice{
                    self.alpm.trans_prepare().expect("couldn't prepare transaction");
                    self.alpm.trans_commit().expect("couldn't run transaction");
                    self.alpm.trans_release().expect("couldn't release transaction");
                }
            }
        }
    }
}