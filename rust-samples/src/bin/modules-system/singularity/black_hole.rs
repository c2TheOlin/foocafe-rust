use self::event_horizon::expand;
use crate::singularity::accretion;
    
pub fn feed() {
    crate::singularity::black_hole::accretion_disk::add_material();
    accretion::add_material();

    event_horizon::expand();
}
  
pub mod event_horizon
{
    pub fn expand(){
        println!("zooooooop")
    }
}

pub mod accretion_disk {
    pub fn add_material()
    {
        println!("Om nom nom")
    }
}