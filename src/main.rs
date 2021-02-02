use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("file: {}", filename);

    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    let mut memory: Vec<i32> = Vec::new();
    let mut index = 0;

    for c in contents.chars() {
        println!("{} {}", c, index);

        if c == '>' {
           index += 1;
           if memory.len() < index {
            memory.push(0);
           }
        }

        if c == '<' {
            index -= 1;
         }

        if c == '+' {
            memory[index - 1] += 1;
        }

        if c == '-' {
            memory[index - 1] -= 1;
        }

        println!("{:#?}", memory);

    }

}
