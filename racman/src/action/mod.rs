pub mod update;

use alpm::Error;
use alpm::Alpm;

pub trait Action {
    fn run(&self,alpm:&mut Alpm)->Result<(),Error>;
}