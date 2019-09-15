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


 pub fn compare_size_explicit<'a>(city: &'a City, city2: &'a City) -> (&'a City) {
        if &city.size_in_sqm <= &city2.size_in_sqm {
                city2
            } else {
                city
            }
    }