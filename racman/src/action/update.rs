use alpm::{Alpm};
use crate::action::Action;
use anyhow::Result;

#[derive(Clone)]
pub struct UpdateAction{
}

impl Action for UpdateAction {
    fn run(&self,alpm:&mut Alpm)->Result<()>{
        for mut db in alpm.syncdbs_mut(){
            db.update(true)?;
        }
        Ok(())
    }
}