use alpm::Error;
use alpm::Alpm;
use crate::transaction::Transaction;
use anyhow::{Result,Context};

#[derive(Clone)]
pub struct InstallTransaction{
    pub name:String
}

impl Transaction for InstallTransaction {
    fn add(&self,alpm:&mut Alpm)->Result<()>{
        let db = alpm.syncdbs().find(|db| db.pkg(&self.name).is_ok()).ok_or_else(||Error::PkgRepoNotFound)?;
        let package = db.pkg(&self.name)?;
        alpm.trans_add_pkg(package).context("couldn't add pkg to transaction")?;
        Ok(())
    }
}
