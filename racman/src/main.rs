mod transaction;
mod action;
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
use clap::{App, load_yaml};
use std::path::PathBuf;
use anyhow::{Context,Result};

fn main() ->Result<()>{
    let cli = load_yaml!("./cli.yaml");
    let matches = App::from_yaml(cli).get_matches();
    let root_dir = PathBuf::from(matches.value_of("root_dir").unwrap_or("/"));
    let db_dir = PathBuf::from(matches.value_of("db_dir").unwrap_or("/var/lib/pacman"));

    let mut racman = Racman::new(root_dir,db_dir).context("could not create racman instance")?;
    racman.set_eventcb(eventcb);
    racman.set_logcb(logcb);
    racman.set_questioncb(questioncb);
    racman.set_progresscb(progresscb);
    racman.set_transaction_confirmationcb(transaction_confirmationcb);
    racman.register_syncdb("core", "http://mirrors.kernel.org/archlinux/core/os/x86_64/").context("could not register syncdb")?;
    racman.register_syncdb("extra", "http://mirrors.kernel.org/archlinux/extra/os/x86_64/").context("could not register syncdb")?;
    racman.register_syncdb("community", "http://mirrors.kernel.org/archlinux/community/os/x86_64/").context("could not register syncdb")?;

    if let Some(matches) = matches.subcommand_matches("install") {
        let syncdb = matches.value_of("SYNCDB").context("No syncdb selected")?;
        let package = matches.value_of("PKG").context("No package selected")?;
        racman.add_install(syncdb,package);
    }
    if let Some(matches) = matches.subcommand_matches("uninstall") {
        let package = matches.value_of("PKG").context("No package selected")?;
        racman.add_remove(package);
    }
    if let Some(_matches) = matches.subcommand_matches("update") {
        racman.add_update();
    }
    if let Some(_matches) = matches.subcommand_matches("upgrade") {
        racman.add_upgrade();
    }
    racman.commit().context("could not commit")?;
    Ok(())
}