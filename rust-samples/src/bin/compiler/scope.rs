pub fn scopes() {

    // Move (like a shallow copy but - no posibility for double free error
    let description = String::from("Hello");
    let my_new_description_pointer = description; // description losed ownership and is dropped here as "Moved" unless cloned
    println!("{}", my_new_description_pointer);
    //description.to_uppercase(); // --> Error
    
    // Borrowed references dont lose ownership
    let trimmed = borrowed_references(&my_new_description_pointer);
    println!("{} {}", trimmed, &my_new_description_pointer);

    // Ownership transfered to the 
    takes_ownership(my_new_description_pointer); // -> my_new_description_pointer within the scope dies
    //println!("{}", my_new_description_pointer);


    let number = 32;
    let other_number = number;
    let result = has_copy_trait(other_number);
    println!("{} {} {}", &number, &other_number, &result);
    
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

