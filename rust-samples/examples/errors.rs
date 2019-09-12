
fn main() {

    let result = Some(5);
    let fail = None;
    let result = do_something(result);
    println!("{}", result.unwrap());

    do_something_bad(fail);
    println!("{}", result.unwrap());
}

fn do_something(val: Option<i32>) ->  Option<i32> {
  match val {
      Some(i) => Some(i * i),
      None => None
  }
}

fn do_something_bad(val: Option<i32>) {
     val.unwrap(); // Only catch is unwrap
}
