pub mod install;
pub mod remove;
pub mod upgrade;
use anyhow::Result;

use alpm::Alpm;

pub trait Transaction {
    fn add(&self,alpm:&mut Alpm)->Result<()>;
}