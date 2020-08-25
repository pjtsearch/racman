mod transaction;
mod cbs;
mod racman;
mod set_cbs;
use crate::cbs::transaction_confirmation::transaction_confirmationcb;
use crate::cbs::progress::progresscb;
use crate::cbs::question::questioncb;
use crate::cbs::log::logcb;
use crate::cbs::event::eventcb;
use crate::set_cbs::SetCBs;
use racman::Racman;

fn main() {
    match Racman::new() {
        Ok(mut racman)=>{
            racman.set_eventcb(eventcb);
            racman.set_logcb(logcb);
            racman.set_questioncb(questioncb);
            racman.set_progresscb(progresscb);
            racman.set_transaction_confirmationcb(transaction_confirmationcb);
            racman.register_syncdb("core", "http://mirrors.evowise.com/archlinux/core/os/x86_64/");
            racman.register_syncdb("extra", "http://mirrors.evowise.com/archlinux/extra/os/x86_64/");
            racman.register_syncdb("community", "http://mirrors.evowise.com/archlinux/community/os/x86_64/");
            // racman.add_upgrade();
            racman.add_install("community","nodejs");
            // racman.add_remove("vi");
            // racman.add_install("core", "perl");
            // racman.add_install("core", "vi");
            // racman.add_install("core", "python-audit");
            racman.commit_transaction();
        },
        Err(error)=>panic!(error)
    }
}