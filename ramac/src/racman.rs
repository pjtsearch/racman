use crate::cbs::progress::progresscb;
use crate::cbs::question::questioncb;
use crate::cbs::fetch::fetchcb;
use crate::cbs::event::eventcb;
use crate::cbs::log::logcb;

use crate::transaction::{Transaction};
use crate::transaction::install::{InstallTransaction};
use crate::transaction::upgrade::{UpgradeTransaction};
use crate::transaction::remove::{RemoveTransaction};

use std::rc::Rc;
use alpm::{Alpm,TransFlag,Progress,SigLevel,set_logcb,set_eventcb,set_fetchcb,set_questioncb,set_progresscb};

pub struct Racman {
    alpm:Alpm,
    transactions:Vec<Rc<dyn Transaction>>
}

impl Racman {
    pub fn new<'a>()->Result<Racman,alpm::Error>{     
        match Alpm::new("/","/var/lib/pacman") {
            Ok(alpm)=>{
                set_logcb!(alpm, logcb);
                set_eventcb!(alpm, eventcb);
                set_fetchcb!(alpm, fetchcb);
                set_questioncb!(alpm, questioncb);
                set_progresscb!(alpm, progresscb);
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
        self.alpm.trans_prepare().expect("couldn't prepare transaction");
        self.alpm.trans_commit().expect("couldn't run transaction");
        self.alpm.trans_release().expect("couldn't release transaction");
    }
}