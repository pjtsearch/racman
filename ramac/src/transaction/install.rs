use alpm::Alpm;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct InstallTransaction{
    pub repo_name:String,
    pub name:String
}

impl Transaction for InstallTransaction {
    fn add(&self,alpm:&mut Alpm){
        let db = alpm.syncdbs().find(|db| db.name() == self.repo_name).unwrap();
        let package = db.pkg(&self.name).unwrap();
        alpm.trans_add_pkg(package).expect("couldn't add pkg to transaction");
    }
}
