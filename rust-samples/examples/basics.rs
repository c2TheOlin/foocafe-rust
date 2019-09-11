use rand::prelude::*;

// This is the common function style - snake case
fn snake_case_with_egyptian_brackets() {
    println!("blah blah parseltongue")
}

// Not too much revolutionary here in the definition of reference types
struct Building {
    name: String,
    number: u32,
    owner: String,
    location: String,
    valdermort_controlled: bool
}

// you can seperate out your impl - Convention based on name
impl Building {

    // normal function
    pub fn get_name(&self) -> &String {
        &self.name
    }

    // associated function
    fn generic_shop() -> Building { 
            Building {
            name: String::from("A boring magic shop"),
            number: random(), 
            owner: String::from("Some person"),
            location: String::from("Diagon alley"),
            valdermort_controlled: false 
        }
    }
}

// auto param name mathcing on the new object
fn create_building_from_template(template: Building, name: String, is_valdermort_controlled: bool) -> Building {
    Building {
        name,
        valdermort_controlled: is_valdermort_controlled,
        location: String::from("UK"),
        ..template //auto populate based on another object
    }
}

fn main() {

    let template = Building { 
        name: String::from("Ministry of Magic"),
        number: 1, 
        owner: String::from("Cornelius Fudge"),
        location: String::from("London"),
        valdermort_controlled: true
    };

    let hogwarts = create_building_from_template(
        template,
        String::from("Hogwarts"),
        false);

    // associated function
    let a_shop = Building::generic_shop();

    println!("{}", a_shop.get_name());

    // Pretty familiar loop/control constructs
    let condition = true;
    let mut counter = if condition { 0 } else { 1 };
    
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