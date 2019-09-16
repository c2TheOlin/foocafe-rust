mod compiler;
use compiler::lifetime_elision::*;
use compiler::scope::show_scopes;

fn main() {
    let city = City::new(54);
    let result;
        
    if city.size_in_sqm > 50 {
        let city2 = City::new(30);
        result = compare_size_explicit(&city, &city2);
    } else {
        let city2 = City::new(30);
        result = compare_size_explicit(&city, &city2);
    }

    println!("{}", result.size_in_sqm);


    // let city2 = City::new(54);
    // let result;
        
    // if city2.size_in_sqm > 50 {
    //     let city = City::new(30);
    //     result = compare_size_explicit_alt(&city, &city2);
    // } else {
    //     let city = City::new(40);
    //     result = compare_size_explicit_alt(&city, &city2);
    // }

    println!("{}", result.size_in_sqm);

    show_scopes();
}