use std::io;
use std::process;

fn main() {
    println!("What is it?");
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("入力エラー");
        println!("{}", input);

        let word = input.trim();
        if word.starts_with("bye") {
            process::exit(0);
        }
    }
}
