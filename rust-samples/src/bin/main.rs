mod compiler;
use compiler::lifetime_elision::*;

fn main() {
    let city = City::new(54);
    let city2 = City::new(46);
    
    let result = city.compare_size_explicit(&city2);
    
    println!("{}", result.size_in_sqm);
}