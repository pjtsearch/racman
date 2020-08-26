pub mod update;

use alpm::Alpm;

pub trait Action {
    fn run(&self,alpm:&mut Alpm);
}