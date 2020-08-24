use alpm::Progress;
use alpm::Question;
use alpm::FetchCbReturn;
use alpm::Event;
use alpm::LogLevel;
use std::rc::Rc;
use alpm::{Alpm,TransFlag,SigLevel,set_logcb,set_eventcb,set_fetchcb,set_questioncb,set_progresscb};

fn main() {
    match Racman::new() {
        Ok(mut racman)=>{
            racman.register_syncdb("core", "http://mirrors.evowise.com/archlinux/core/os/x86_64/");
            racman.register_syncdb("extra", "http://mirrors.evowise.com/archlinux/extra/os/x86_64/");
            racman.register_syncdb("community", "http://mirrors.evowise.com/archlinux/community/os/x86_64/");
            racman.add_upgrade();
            racman.add_install("core","nano");
            // racman.add_remove("vi");
            // racman.add_install("core", "perl");
            // racman.add_install("core", "vi");
            // racman.add_install("core", "python-audit");
            racman.commit_transaction();
        },
        Err(error)=>panic!(error)
    }
}

trait Transaction {
    fn add(&self,alpm:&mut Alpm);
}

#[derive(Clone)]
struct InstallTransaction{
    repo_name:String,
    name:String
}

impl Transaction for InstallTransaction {
    fn add(&self,alpm:&mut Alpm){
        let db = alpm.syncdbs().find(|db| db.name() == self.repo_name).unwrap();
        let package = db.pkg(&self.name).unwrap();
        alpm.trans_add_pkg(package).expect("couldn't add pkg to transaction");
    }
}


#[derive(Clone)]
struct UpgradeTransaction{
}

impl Transaction for UpgradeTransaction {
    fn add(&self,alpm:&mut Alpm){
        let mut will_upgrade = false;
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

#[derive(Clone)]
struct RemoveTransaction{
    name:String
}

impl Transaction for RemoveTransaction {
    fn add(&self,alpm:&mut Alpm){
        let db = alpm.localdb();
        let package = db.pkg(&self.name).unwrap();
        alpm.trans_remove_pkg(package).expect("couldn't add pkg to transaction");
    }
}

struct Racman {
    alpm:Alpm,
    transactions:Vec<Rc<dyn Transaction>>
}

impl Racman {
    fn new<'a>()->Result<Racman,alpm::Error>{
        fn logcb(level: LogLevel, msg: &str) {
            if level == LogLevel::ERROR {
                print!("log {}", msg);
            }
        }
        fn eventcb(event: &Event) {
            match event {
                Event::DatabaseMissing(x) => println!("missing database: {}", x.dbname()),
                _ => println!("event: {:?}", event),
            }
        }
    
        fn fetchcb(_url: &str, _path: &str, _force: bool) -> FetchCbReturn {
            FetchCbReturn::Ok
        }
    
        fn questioncb(question: &Question) {
            println!("question {:?}", question);
            match question {
                Question::Conflict(x) => {
                    let c = x.conflict();
                    println!("CONFLICT BETWEEN {} AND {}", c.package1(), c.package2(),);
                    println!("conflict: {}", c.reason());
                }
                _ => (),
            }
        }
    
        fn progresscb(progress: Progress, pkgname: &str, percent: i32, howmany: usize, current: usize) {
            println!(
                "progress {:?}, {} {} {} {}",
                progress, pkgname, percent, howmany, current
            );
        }
    
        match Alpm::new("/","/var/lib/pacman") {
            Ok(alpm)=>{
                set_logcb!(alpm, logcb);
                set_eventcb!(alpm, eventcb);
                set_fetchcb!(alpm, fetchcb);
                set_questioncb!(alpm, questioncb);
                set_progresscb!(alpm, progresscb);
                Ok(Racman {
                    alpm,
                    transactions:vec![]
                })
            },
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
    fn add_upgrade(&mut self){
        self.transactions.push(Rc::new(UpgradeTransaction{}));
    }
    fn add_remove(&mut self,name:&str){
        self.transactions.push(Rc::new(RemoveTransaction{name:name.to_owned()}));
    }
    fn commit_transaction(&mut self){
        let add_transactions = |transactions:&Vec<Rc<dyn Transaction>>,alpm:&mut Alpm| {
            transactions.iter().for_each(|transaction|{
                transaction.add(alpm);
            });
        };
        self.alpm.trans_init(TransFlag::NONE).expect("couldn't init transaction");
        add_transactions(&self.transactions,&mut self.alpm);
        self.alpm.trans_prepare().expect("couldn't prepare transaction");
        self.alpm.trans_commit().expect("couldn't run transaction");
        self.alpm.trans_release().expect("couldn't release transaction");
    }
}