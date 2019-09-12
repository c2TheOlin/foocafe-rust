fn main() {
    let myAnswer = NeosChoice::BluePill(String::from("awesooooome"));
    myAnswer.response();

    let i_know_kungfu_answer = NeosChoice::SlapMorpheus(65);
    i_know_kungfu_answer.response();

    let neosAnswer = NeosChoice::RedPill;
    neosAnswer.response();

    neosAnswer.match_response();

    let not_interested = NeosChoice::SomethingElse;
    not_interested.match_response();
}

enum NeosChoice {
    RedPill,
    BluePill(String),
    SlapMorpheus(u8),
    SomethingElse
}

// if based
impl NeosChoice {
    fn response(&self) {
        if let NeosChoice::BluePill(_i) = self {
            println!("Being a regular coder is {}", _i);
        }

        // This is manual and not compiler checked
        if let NeosChoice::SlapMorpheus(_i) = self {
            println!("Slap him {} times", _i);        
        }

       if let NeosChoice::RedPill = self {
            println!("Lets see how far the rabbit hole goes");    
        }   
    }

    // This is exhaustive
    fn match_response(&self) {       
        match self {
            NeosChoice::BluePill(message) =>   println!("Being a regular coder is {}", message),
            NeosChoice::SlapMorpheus(count) => println!("Slap him {} times", count),
            NeosChoice::RedPill => println!("Lets see how far the rabbit hole goes"),
            _ => println!("Ill take the red pill but only if I get an iphone with 3 cameras")
        }        
    }
}
