// let convo = chat("hi there, how are you", &["hi im good", "hi im bad", "bye"]);
// match choice {
//     1 => println!("correct reponse"),
//     2 => println!("ur ruining my vibe"),
//     3 => println!("what?"),
//     _ => unreachable!(),
// }

use std::io::{self, Write};

pub fn chat(message: &str, options: &[&str]) -> usize {
    println!("{}", message);
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }

    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<usize>() {
        Ok(num) if num >= 1 && num <= options.len() => num,
        _ => {
            println!("invalid option.");
            chat(message, options) // retry on fail
        }
    }
}
