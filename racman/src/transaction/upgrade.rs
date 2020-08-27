use alpm::Alpm;
use crate::transaction::Transaction;
use anyhow::Result;

#[derive(Clone)]
pub struct UpgradeTransaction{
}

impl Transaction for UpgradeTransaction {
    fn add(&self,alpm:&mut Alpm)->Result<()>{
        for db in alpm.syncdbs(){
            let local_pkgs = alpm.localdb().pkgs()?;
            for pkg in local_pkgs{
                if let Ok(db_pkg)=db.pkg(pkg.name()) {
                    if db_pkg.version() != pkg.version(){
                        alpm.trans_add_pkg(db_pkg)?;
                    }
                }
            }
        }
        Ok(())
    }
}