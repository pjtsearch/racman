use std::rc::Rc;
use alpm::{Alpm,TransFlag,SigLevel};

fn main() {
    match Racman::new() {
        Ok(mut racman)=>{
            racman.register_syncdb("core", "http://mirrors.evowise.com/archlinux/core/os/x86_64/");
            racman.add_install("core", "vi");
            racman.commit_transaction();
        },
        Err(error)=>panic!(error)
    }
}

trait Transaction {
    fn commit(&self,alpm:&mut Alpm);
}

#[derive(Clone)]
struct InstallTransaction{
    repo_name:String,
    name:String
}

impl Transaction for InstallTransaction {
    fn commit(&self,alpm:&mut Alpm){
        let db = alpm.syncdbs().find(|db| db.name() == self.repo_name).unwrap();
        let package = db.pkg(&self.name).unwrap();
        alpm.trans_init(TransFlag::NONE).expect("couldn't init transaction");
        alpm.trans_add_pkg(package).expect("couldn't add pkg to transaction");
        alpm.trans_prepare().expect("couldn't prepare transaction");
        alpm.trans_commit().expect("couldn't run transaction");
    }
}

struct Racman {
    alpm:Alpm,
    transactions:Vec<Rc<dyn Transaction>>
}

impl Racman {
    fn new<'a>()->Result<Racman,alpm::Error>{
        match Alpm::new("/","/var/lib/pacman") {
            Ok(alpm)=>Ok(Racman {
                alpm,
                transactions:vec![]
            }),
            Err(err)=>Err(err)
        }
    }
    fn register_syncdb(&mut self,repo_name:&str,server:&str){
        let syncdb = self.alpm.register_syncdb_mut(repo_name, SigLevel::NONE).unwrap();
        syncdb.add_server(server)
            .unwrap();
    }
    fn add_install(&mut self,repo_name:&str,name:&str){
        self.transactions.push(Rc::new(InstallTransaction{repo_name:repo_name.to_owned().clone(),name:name.to_owned().clone()}));
    }
    fn commit_transaction(&mut self){
        let commit = |transactions:&Vec<Rc<dyn Transaction>>,alpm:&mut Alpm| {
            transactions.iter().for_each(|transaction|{
                transaction.commit(alpm);
            });
        };
        commit(&self.transactions,&mut self.alpm);
    }
}