use alpm::Error;
use alpm::Alpm;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct RemoveTransaction{
    pub name:String
}

impl Transaction for RemoveTransaction {
    fn add(&self,alpm:&mut Alpm)->Result<(),Error>{
        let db = alpm.localdb();
        let package = db.pkg(&self.name)?;
        alpm.trans_remove_pkg(package)?;
        Ok(())
    }
}