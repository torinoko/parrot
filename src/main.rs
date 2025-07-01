use std::io;
use std::process;

fn main() {
    println!("What is it?");
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("入力エラー");

        let word = input.trim();
        if word.starts_with("bye") {
            println!("Goodbye! See you again!");
            process::exit(0);
        }
        println!("{}", word);
    }
}
