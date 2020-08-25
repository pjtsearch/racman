use alpm::Alpm;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct RemoveTransaction{
    pub name:String
}

impl Transaction for RemoveTransaction {
    fn add(&self,alpm:&mut Alpm){
        let db = alpm.localdb();
        let package = db.pkg(&self.name).unwrap();
        alpm.trans_remove_pkg(package).expect("couldn't add pkg to transaction");
    }
}