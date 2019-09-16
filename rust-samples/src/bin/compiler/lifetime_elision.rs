pub struct City {
    pub size_in_sqm: u32
}

impl City {
    pub fn new(size: u32) -> City {
        City { size_in_sqm: size }
    }
}

// pub fn compare_size(city: &City, city2: &City) -> &City {
//     if &city.size_in_sqm <= &city2.size_in_sqm {
//             city2
//             } else {
//             city
//         }
// }

// since either city can be returned the input references must have the same lifetime to show relation
 pub fn compare_size_with_lifetimes<'a>(city: &'a City, city2: &'a City) -> (&'a City) {
    if &city.size_in_sqm <= &city2.size_in_sqm {
            city2
        } else {
            city
        }
}

 pub fn do_something_with_cities<'a, 'b, 'c, T>(city: &'a T, city2: &'b T, city3: &'c T) -> (&'b T) {
    //do something with other cities but retrun city2
    city2
}