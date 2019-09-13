fn main() {
    /* 4 Main Collections/Compound Types*/

    // Tuple
    let tuple: (i32, f64, &str) = (500, 6.4, "asd"); // Pack
    let (unsigned_int, float, slice) = tuple; // Unpack
    
    // Tuple Struct - extra meaning
    struct Coordinates(i32, i32);
    let coord = Coordinates(4,4);

    if tuple.0 == unsigned_int {
        println!("Tuple Tuple Tuple")
    }
    
    // Arrays - value type - have traits and are on stack
    let mut my_array: [u32; 6] = [5, 3, 2, 1, 1, 0];
    let another_array = [2; 2];
    let infered_array = ["Walk", "Like", "An", "Egyptian"];
    let slice_array: &[u32] = &my_array[..2];
   
    //let x = my_array[200] this panics!

    for val in &my_array {
        println!("Fib {}", val);
    }

    for val in slice_array {
        println!("Sliced {}", val);
    }

    my_array.reverse();

    let mapped = my_array.iter().map(|val| val + 1); 
    for val in mapped {
        println!("{}", val);
    }
    
    // Vectors
    let mut my_vector: Vec<bool> = Vec::new();
    let my_macro_vec = vec!["Jazz", "Rock", "Classical"];

    my_vector.push(true);
    let value = &my_vector[0]; 
    println!("{}", value);

    for v in &my_macro_vec {
        println!("{}", v);
    }

    // Also hash maps
}
