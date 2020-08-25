use alpm::Package;
use alpm::AlpmList;
use quest::yesno;

use std::io;
use std::io::Write;

pub fn transaction_confirmationcb(adding:AlpmList<Package>,removing:AlpmList<Package>)->bool{
    println!("Transaction Summary:");
    println!("To be added:");
    adding.into_iter().for_each(|pkg|{
        print!("{}-{} ",pkg.name(),pkg.version())
    });
    println!();
    println!("To be removed:");
    removing.into_iter().for_each(|pkg|{
        print!("{}-{} ",pkg.name(),pkg.version())
    });
    println!();
    print!("Commit transaction? [y/N]:");
    io::stdout().flush().unwrap();
    let question = yesno(false);
    if let Ok(opt) = question {
        if let Some(choice) = opt{
            choice
        }else {
            false
        }
    }else{
        false
    }
}