mod compiler;
use compiler::lifetime_elision::*;

//Same scope for references everything is nice!
fn main() {
    let city = City::new(54);      
    let city2 = City::new(30);

    let result = compare_size_with_lifetimes(&city, &city2);
   
    println!("{}", result.size_in_sqm);
}

// Calling this means that city2 is in a difference lifetime 
// fn main() {
//     let city = City::new(54);
//     let want_to_do_something = true;
//     let result;

//     if(want_to_do_something) {
//         let city2 = City::new(30);
//         result = compare_size_with_lifetimes(&city, &city2);
//     }
  
//     println!("{}", result.size_in_sqm);
// }


// // Calling these will work since result, city and city2 are used in the same of greater lifetime
// fn main() {
//     let city = City::new(54);
//     let want_to_do_something = true;
//     let result;

//     if(want_to_do_something) {
//         let city2 = City::new(30);
//         result = compare_size_with_lifetimes(&city, &city2);
//         println!("{}", result.size_in_sqm);
//     } 
// }

// // Calling this will work since result, city and city2 are used in the same of greater lifetime
// fn main() {
//     let city = City::new(54);
//     let want_to_do_something = true;
//     let result;
//     let city2;

//     if(want_to_do_something) {
//         city2 = City::new(30);
//         result = compare_size_with_lifetimes(&city, &city2);
//     } else {
//         result = compare_size_with_lifetimes(&city, &city); // this is a work around 
//     }

//     println!("{}", result.size_in_sqm);
// }

// // like wise if only one lifetime is relevant that is the only one enforces
// fn main() {
//     let city = City::new(54);
//     let city3 = City::new(54);
//     let want_to_do_something = true;
//     let result;
  
//     if(want_to_do_something) {
//         let city2 = City::new(30);
//         result = do_something_with_cities(&city, &city2, &city);
//         // --> println! here
//     } 

//     println!("{}", result.size_in_sqm);
// }

// fn main() {
//     let city2 = City::new(30);  
//     let want_to_do_something = true;
//     let mut result = &City::new(0);
  
//     if(want_to_do_something) {
//         let city = City::new(54);
//         let city3 = City::new(54);
//         result = do_something_with_cities(&city, &city2, &city);
//         // --> println! here
//     } 

//     println!("{}", result.size_in_sqm);
// }