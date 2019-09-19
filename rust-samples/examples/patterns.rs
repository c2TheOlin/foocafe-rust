fn main() {
    let end = 10;
    let closure = |start| {
         for val in (start..end).rev() {
            println!("{}", val);
        }
    };

    closure(0);
    
    // Pattern matching
    let music_style= vec!["Experimental", "Hard", "Modern"];
    let music_type = vec!["Jazz", "Rock", "Classical"];

    let my_music: Vec<String> = music_style
                    .iter()
                    .zip(music_type)
                    .map(|(a, b)| format!("{} {}",a,b))
                    .filter(|val| !val.contains("Jazz"))
                    .collect();

    for val in my_music {
        println!("{}", val);
    }


}
