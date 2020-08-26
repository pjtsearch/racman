use alpm::Alpm;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct UpgradeTransaction{
}

impl Transaction for UpgradeTransaction {
    fn add(&self,alpm:&mut Alpm){
        let mut will_upgrade = false;
        for mut db in alpm.syncdbs_mut(){
            // println!("{}", db.name());
            db.update(true).expect("failed to update");
            // if db.pkg("libx11").is_ok() {
            //     println!("{}", db.pkg("libx11").unwrap().version());
            // }
        }
        for db in alpm.syncdbs(){
            let local_pkgs = alpm.localdb().pkgs().unwrap();
            local_pkgs.into_iter().for_each(|pkg|{
                if let Ok(db_pkg)=db.pkg(pkg.name()) {
                    if db_pkg.version() != pkg.version(){
                        alpm.trans_add_pkg(db_pkg).expect("couldn't add pkg to transaction");
                        will_upgrade = true;
                    }
                }
            });
        }
    }
}