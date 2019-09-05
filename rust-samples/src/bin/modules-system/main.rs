mod singularity;

use self::singularity::black_hole;
use std::io::{self, Write};
use std::collections::*;

fn main() {
    black_hole::feed();
    black_hole::accretion_disk::add_material();
}