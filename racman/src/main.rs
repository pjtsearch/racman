mod transaction;
mod cbs;
mod racman;
use racman::Racman;

fn main() {
    match Racman::new() {
        Ok(mut racman)=>{
            racman.register_syncdb("core", "http://mirrors.evowise.com/archlinux/core/os/x86_64/");
            racman.register_syncdb("extra", "http://mirrors.evowise.com/archlinux/extra/os/x86_64/");
            racman.register_syncdb("community", "http://mirrors.evowise.com/archlinux/community/os/x86_64/");
            // racman.add_upgrade();
            racman.add_install("community","xed");
            // racman.add_remove("vi");
            // racman.add_install("core", "perl");
            // racman.add_install("core", "vi");
            // racman.add_install("core", "python-audit");
            racman.commit_transaction();
        },
        Err(error)=>panic!(error)
    }
}