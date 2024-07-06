use std::io;

fn main() {
    let mut var = String::new();
    io::stdin().read_line(&mut var)
	.expect("Failed to read line");
    println!("Hello, {}", var);
}
