use alpm::{Alpm};
use crate::action::Action;

#[derive(Clone)]
pub struct UpdateAction{
}

impl Action for UpdateAction {
    fn run(&self,alpm:&mut Alpm){
        for mut db in alpm.syncdbs_mut(){
            db.update(true).expect("failed to update");
        }
    }
}