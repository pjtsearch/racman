use alpm::CommitReturn;
use alpm::PrepareReturn;
use alpm::Error;
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

use anyhow::{Context,Result};

pub struct Racman {
    pub alpm:Alpm,
    transactions:Vec<Rc<dyn Transaction>>,
    actions:Vec<Rc<dyn Action>>,
    pub cbs:CBs
}

impl Racman {
    pub fn new<'a>(root_dir:PathBuf,db_dir:PathBuf)->Result<Racman>{     
        let alpm = Alpm::new(
            root_dir.to_str().ok_or_else(||Error::NotADir).context("root dir not a dir")?,
            db_dir.to_str().ok_or_else(||Error::NotADir).context("db dir not a dir")?
        )?;
        Ok(Racman {
            alpm,
            transactions:vec![],
            actions:vec![],
            cbs:CBs::default()
        })
    }
    pub fn register_syncdb(&mut self,repo_name:&str,server:&str)->Result<()>{
        let mut syncdb = self.alpm.register_syncdb_mut(repo_name, SigLevel::NONE)?;
        syncdb.add_server(server)?;
        syncdb.update(false)?;
        Ok(())
    }
    pub fn add_install(&mut self,name:&str){
        self.transactions.push(Rc::new(InstallTransaction{name:name.to_owned().clone()}));
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
    pub fn commit_transactions(&mut self)->Result<()>{
        let add_transactions = |transactions:&Vec<Rc<dyn Transaction>>,alpm:&mut Alpm|->Result<()> {
            transactions.iter().map(|transaction|{
                transaction.add(alpm)
            }).collect::<Result<()>>()
        };
        self.alpm.trans_init(TransFlag::NONE)?;
        add_transactions(&self.transactions,&mut self.alpm)?;
        if (self.cbs.transaction_confirmationcb)(self.alpm.trans_add(),self.alpm.trans_remove()){
            self.alpm.trans_prepare().or_else(|error:(PrepareReturn, Error)|Err(error.1))?;
            self.alpm.trans_commit().or_else(|error:(CommitReturn, Error)|Err(error.1))?;
            self.alpm.trans_release()?;
        }
        Ok(())
    }
    pub fn commit_actions(&mut self)->Result<()>{
        let add_actions = |actions:&Vec<Rc<dyn Action>>,alpm:&mut Alpm|->Result<()> {
            actions.iter().map(|action|{
                action.run(alpm)
            }).collect::<Result<()>>()
        };
        add_actions(&self.actions,&mut self.alpm)?;
        Ok(())
    }
    pub fn commit(&mut self)->Result<()>{
        if self.actions.len() > 0{
            self.commit_actions()?;
        }
        if self.transactions.len() > 0{
            self.commit_transactions()?;
        }
        Ok(())
    }
}