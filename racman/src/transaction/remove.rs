use alpm::Alpm;
use crate::transaction::Transaction;
use anyhow::Result;

#[derive(Clone)]
pub struct RemoveTransaction{
    pub name:String
}

impl Transaction for RemoveTransaction {
    fn add(&self,alpm:&mut Alpm)->Result<()>{
        let db = alpm.localdb();
        let package = db.pkg(&self.name)?;
        alpm.trans_remove_pkg(package)?;
        Ok(())
    }
}