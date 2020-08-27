pub mod update;

use anyhow::Result;
use alpm::Alpm;

pub trait Action {
    fn run(&self,alpm:&mut Alpm)->Result<()>;
}