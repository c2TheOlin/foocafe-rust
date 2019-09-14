use std::io;
use std::error::Error;

// 
fn main()  -> Result<(), Box<dyn Error>> {

    //Option<T>
    let result = Some(5); // some method value
    let fail = None;
    let result = do_something(result);
    println!("{}", result.unwrap_or_else(|| 666));

    println!("{}", fail.unwrap_or_else(|| 666));
   // fail.unwrap(); // Only catch is unwrap

    // Result <T, E>
    // Match
    let result = match do_something_else() {
        Ok(v) => Ok(v),
        Err(e) => Err(e)
    };

    // but this can be shorthanded to 
    let result = do_something_else()?;
    println!("{:?}", result);
    Ok(())
}


fn do_something(val: Option<i32>) ->  Option<i32> {
  match val {
      Some(i) => Some(i * i),
      None => None
  }
}

fn do_something_else() -> Result<i32, io::Error> {
    return Result::Err(io::Error::from(io::ErrorKind::ConnectionAborted))
}
