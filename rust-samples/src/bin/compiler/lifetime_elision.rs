pub struct City {
    pub size_in_sqm: u32
} 

impl City {
    pub fn new(size: u32) -> City {
        City { size_in_sqm: size }
    }

    // pub fn compare_size(&self, city: &City) -> &City {
    //     if &self.size_in_sqm <= &city.size_in_sqm {
    //             city
    //         } else {
    //             self
    //         }
    // }

    pub fn compare_size_explicit<'a>(&'a self, city: &'a City) -> (&'a City) {
        if &self.size_in_sqm <= &city.size_in_sqm {
                city
            } else {
                self
            }
    }
}

pub fn sum_size(city1: &u32, city2: &u32) -> u32 {
    city1 + city2
}

pub fn sum_size_explicit<'a, 'b>(city1: &'a u32, city2: &'b u32) -> u32 {
    city1 + city2
}