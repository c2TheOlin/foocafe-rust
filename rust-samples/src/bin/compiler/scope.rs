pub fn show_scopes() {

    // Move (like a shallow copy but - no posibility for double free error
    let mut description = String::from("Hello");
    let my_new_description_pointer = description; // description dies here as "Moved" unless cloned
    println!("{}", my_new_description_pointer);
    //description.to_uppercase(); // --> Error

    let trimmed = borrowed_references(&my_new_description_pointer);
    println!("{} {}", trimmed, &my_new_description_pointer);

    takes_ownership(my_new_description_pointer); // -> my_new_description_pointer dies
    //println!("{}", my_new_description_pointer);

    let number = 32;
    let other_number = number;
    has_copy_trait(other_number);
    println!("{} {}", number, &other_number);
    
} // trimmed, number and other number dies here

fn borrowed_references(text: &str) -> &str{
    &text[..4]   
} // its a reference so no ownership nothing happenss

fn takes_ownership(text: String) {
    ()
} // text dies here

fn has_copy_trait(number: i32) -> i32 {
    number * 2
} // nothing happens

