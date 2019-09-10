use rand::prelude::*;

fn snake_case_with_egyptian_brackets() {
    println!("blah blah parseltongue")
}

fn create_building_from_template(template: Building, name: String, valdermort_controlled: bool) -> Building {
    Building {
        name,
        valdermort_controlled,
        location: String::from("UK"),
        ..template
    }
}

struct Building {
    name: String,
    number: u32,
    owner: String,
    location: String,
    valdermort_controlled: bool
}

impl Building {

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

fn main() {
    snake_case_with_egyptian_brackets();

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
}