use rand::prelude::*;

// Not too much revolutionary here in the definition of reference types
#[derive(Debug, PartialEq)] // deriveable traits
struct Building {
    name: String,
    number: u32,
    owner: String,
    location: String,
    valdermort_controlled: bool
}

// you can seperate out your impl - Convention based on name
impl Building {
    // normal method
    pub fn get_name(&self) -> &String {
        &self.name
    }

    // associated function
     fn build_hogwarts() -> Building { 
            Building {
            name: String::from("Howarts"),
            number: random(), 
            owner: String::from("Dumbledor"),
            location: String::from("UK?"),
            valdermort_controlled: false 
        }
    }
}

// Traits/Exisiting and Custom
pub trait Defend {
    fn cast_protection_spell(&self);
}

impl Defend for Building {
    fn cast_protection_spell(&self) {
        println!("Salvio Hexia");
    }
}
// Trait Bounds
pub fn trait_param(obj: impl Defend) {
    obj.cast_protection_spell();
}

pub fn trait_generics<T: Defend>(obj: T, obj2: T) {
    obj.cast_protection_spell();
    obj2.cast_protection_spell();
}

fn main() {

    let ministry = Building { 
        name: String::from("Ministry of Magic"),
        number: 1, 
        owner: String::from("Cornelius Fudge"),
        location: String::from("London"),
        valdermort_controlled: true
    };

    // associated function
    let hogwarts = Building::build_hogwarts();

    // custom trait
    hogwarts.cast_protection_spell();

    // Derived traits
    println!("{:?}", hogwarts);
    assert_ne!(hogwarts, ministry);

    // Pretty familiar loop/control constructs
    let mut counter = if true { 0 } else { 1 };
    
    loop {
        counter +=1;
        println!("{}", hogwarts.get_name());

        if(counter == 5)
        {
            break;
        }

    }

    while (counter <= 10){
           println!("Harry Potter");
           counter += 1;
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}