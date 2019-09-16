#[derive(Debug, PartialEq)]
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
    do_something_with_other_cities(city, city2);
    if &city.size_in_sqm <= &city2.size_in_sqm {
            city2
        } else {
            city
        }
}

 pub fn do_something_with_cities<'a, 'b, 'c>(city: &'a City, city2: &'b City, city3: &'c City) -> (&'b City) {
    //do something with other cities but retrun city2
    city2
}

fn do_something_with_other_cities<'a, 'b>(city: &'a City,  city2: &'a City) -> u32 {
     city.size_in_sqm + city2.size_in_sqm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_sizes_returns_largest_city()
    {
        let city1 = City::new(2);
        let city2 = City::new(4);

        let result = compare_size_with_lifetimes(&city1, &city2);

        assert_eq!(city2, *result);
    }

    
    #[test]
    fn do_something_with_other_cities_combines_sizes()
    {
        let city1 = City::new(2);
        let city2 = City::new(4);

        let result = do_something_with_other_cities(&city1, &city2);

        assert!(result == 6);
    }
}