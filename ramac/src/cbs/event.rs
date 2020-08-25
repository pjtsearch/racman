use alpm::{Event,EventType,PackageOperation};

pub fn eventcb(event: &Event) {
    match event {
        Event::DatabaseMissing(x) => println!("missing database: {}", x.dbname()),
        Event::PkgDownload(pkg_download) => match pkg_download.event_type(){
            EventType::PkgDownloadStart => println!("Downloading `{}'",pkg_download.file()),
            EventType::PkgDownloadDone => println!("Done downloading `{}'",pkg_download.file()),
            EventType::PkgDownloadFailed => println!("Failed downloading `{}'",pkg_download.file()),
            _ => println!()
        },
        Event::PackageOperation(pkg_operation) => match pkg_operation.operation(){
            PackageOperation::Install(pkg) => println!("Installing {}",pkg.name()),
            PackageOperation::Reinstall(new_pkg,_pkg) => println!("Reinstalling {}",new_pkg.name()),
            PackageOperation::Upgrade(new_pkg,pkg) => println!("Updating {}-{} to {}",pkg.name(),pkg.version(),new_pkg.version()),
            PackageOperation::Downgrade(new_pkg,pkg) => println!("Downgrade {}-{} to {}",pkg.name(),pkg.version(),new_pkg.version()),
            PackageOperation::Remove(pkg) => println!("Removing {}",pkg.name()),
        },
        Event::Other(event) => match event{
            EventType::ResolveDepsStart => println!("Resolving dependencies"),
            EventType::ResolveDepsDone => println!("Done resolving dependencies"),
            EventType::InterConflictsStart => println!("Checking inter-confilicts"),
            EventType::InterConflictsDone => println!("Done checking inter-confilicts"),
            EventType::RetrieveStart => println!("Retrieving packages"),
            EventType::RetrieveDone => println!("Done retrieving packages"),
            EventType::KeyringStart => println!("Getting keyring"),
            EventType::KeyringDone => println!("Done getting keyring"),
            EventType::IntegrityStart => println!("Checking integrity"),
            EventType::IntegrityDone => println!("Done checking integrity"),
            EventType::LoadStart => println!("Loading"),
            EventType::LoadDone => println!("Done loading"),
            EventType::FileConflictsStart => println!("Checking file conflicts"),
            EventType::FileConflictsDone => println!("Done checking file conflicts"),
            EventType::TransactionStart => println!("Running transaction"),
            EventType::TransactionDone => println!("Done running transaction"),
            EventType::HookStart => println!("Running hooks"),
            EventType::HookDone => println!("Done running hooks"),
            EventType::HookRunStart => println!("Running hook"),
            EventType::HookRunDone => println!("Done running hook"),
            _ => println!("event: {:?}", event),
        },
        _ => println!("event: {:?}", event),
    }
}