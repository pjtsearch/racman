use alpm::Error;
use alpm::Alpm;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct InstallTransaction{
    pub repo_name:String,
    pub name:String
}

impl Transaction for InstallTransaction {
    fn add(&self,alpm:&mut Alpm)->Result<(),Error>{
        let db = alpm.syncdbs().find(|db| db.name() == self.repo_name).ok_or_else(||Error::DbNotFound)?;
        let package = db.pkg(&self.name)?;
        alpm.trans_add_pkg(package).expect("couldn't add pkg to transaction");
        Ok(())
    }
}
