use alpm::Db;
use alpm::Pkg;
use std::sync::Mutex;
use std::sync::Arc;
use std::cell::RefCell;
use alpm::{Alpm,TransFlag,SigLevel,Package};

fn main() {
    // let alpm = Alpm::new("/","/var/lib/pacman");
    // match alpm{
    //     Ok(mut alpm)=>{
    //         {get_db(&mut alpm);}
    //         Pk::new(&mut alpm,"vi".to_owned()).install();
    //         // alpm.set_use_syslog(true);

    //         // let pkg = get_pkg(&mut alpm,"vi".to_owned());
    //         // install(&mut alpm, pkg)
    //     },
    //     Err(error)=>println!("{:#?}",error)            
    // }
    match Racman::new() {
        Ok(mut racman)=>{
            racman.register_syncdb("core", "http://mirrors.evowise.com/archlinux/core/os/x86_64/");
            racman.add_install("core", "vi");
            racman.commit_transaction();
        },
        Err(error)=>panic!(error)
    }

    // println!("Hello, world!");
}

fn get_db(alpm:&mut Alpm){
    let db = alpm.register_syncdb_mut("core", SigLevel::NONE).unwrap();
    db.add_server("http://mirrors.evowise.com/archlinux/core/os/x86_64/")
    .unwrap();
}

// fn install(alpm:&mut Alpm,pkg:Package){
//     alpm.trans_init(TransFlag::NONE).expect("couldn't init transaction");
//     alpm.trans_add_pkg(pkg).expect("couldn't add pkg to transaction");
//     alpm.trans_prepare().expect("couldn't prepare transaction");
//     alpm.trans_commit().expect("couldn't run transaction");
// }

struct Pk<'a> {
    alpm:&'a mut Alpm,
    name:String
}

impl Pk<'_> {
    fn new<'a>(alpm:&'a mut Alpm,name:String)->Pk{
        return Pk{
            alpm,
            name
        }
    }
    fn install(&mut self){
        let db = self.alpm.syncdbs().find(|db| db.name() == "core").unwrap();
        let package = db.pkg(&self.name).unwrap();
        self.alpm.trans_init(TransFlag::NONE).expect("couldn't init transaction");
        self.alpm.trans_add_pkg(package).expect("couldn't add pkg to transaction");
        self.alpm.trans_prepare().expect("couldn't prepare transaction");
        self.alpm.trans_commit().expect("couldn't run transaction");
    }
}

trait Transaction {
}

#[derive(Clone)]
struct InstallTransaction{
    repo_name:String,
    name:String
}

impl Transaction for InstallTransaction {}

struct Racman {
    alpm:Alpm,
    transactions:Vec<InstallTransaction>
}

impl Racman {
    fn new()->Result<Racman,alpm::Error>{
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
        let db = self.alpm.syncdbs().find(|db| db.name() == repo_name).unwrap();
        let package = db.pkg(name).unwrap();
        self.transactions.push(InstallTransaction{repo_name:repo_name.to_owned(),name:name.to_owned()});
        // self.alpm.trans_init(TransFlag::NONE).expect("couldn't init transaction");
        // self.alpm.trans_add_pkg(package).expect("couldn't add pkg to transaction");
    }
    fn commit_transaction(&mut self){
        self.alpm.trans_prepare().expect("couldn't prepare transaction");
        self.alpm.trans_commit().expect("couldn't run transaction");
    }
}