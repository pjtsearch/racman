use crate::action::update::UpdateAction;
use crate::action::Action;
use std::path::PathBuf;
use crate::set_cbs::CBs;

use crate::transaction::{Transaction};
use crate::transaction::install::{InstallTransaction};
use crate::transaction::upgrade::{UpgradeTransaction};
use crate::transaction::remove::{RemoveTransaction};

use std::rc::Rc;
use alpm::{Alpm,TransFlag,SigLevel};

pub struct Racman {
    pub alpm:Alpm,
    transactions:Vec<Rc<dyn Transaction>>,
    actions:Vec<Rc<dyn Action>>,
    pub cbs:CBs
}

impl Racman {
    pub fn new<'a>(root_dir:PathBuf,db_dir:PathBuf)->Result<Racman,alpm::Error>{     
        match Alpm::new(root_dir.to_str().expect("Root dir does not exist"),db_dir.to_str().expect("DB dir does not exist")) {
            Ok(alpm)=>{
                Ok(Racman {
                    alpm,
                    transactions:vec![],
                    actions:vec![],
                    cbs:CBs::default()
                })
            },
            Err(err)=>Err(err)
        }
    }
    pub fn register_syncdb(&mut self,repo_name:&str,server:&str){
        let mut syncdb = self.alpm.register_syncdb_mut(repo_name, SigLevel::NONE).unwrap();
        syncdb.add_server(server)
            .unwrap();
        syncdb.update(false).expect("failed to update");
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
    pub fn add_update(&mut self){
        self.actions.push(Rc::new(UpdateAction{}));
    }
    pub fn commit_transactions(&mut self){
        let add_transactions = |transactions:&Vec<Rc<dyn Transaction>>,alpm:&mut Alpm| {
            transactions.iter().for_each(|transaction|{
                transaction.add(alpm);
            });
        };
        self.alpm.trans_init(TransFlag::NONE).expect("couldn't init transaction");
        add_transactions(&self.transactions,&mut self.alpm);
        if (self.cbs.transaction_confirmationcb)(self.alpm.trans_add(),self.alpm.trans_remove()){
            self.alpm.trans_prepare().expect("couldn't prepare transaction");
            self.alpm.trans_commit().expect("couldn't run transaction");
            self.alpm.trans_release().expect("couldn't release transaction");
        }
    }
    pub fn commit_actions(&mut self){
        let add_actions = |actions:&Vec<Rc<dyn Action>>,alpm:&mut Alpm| {
            actions.iter().for_each(|action|{
                action.run(alpm);
            });
        };
        add_actions(&self.actions,&mut self.alpm);
    }
    pub fn commit(&mut self){
        if self.actions.len() > 0{
            self.commit_actions();
        }
        if self.transactions.len() > 0{
            self.commit_transactions();
        }
    }
}