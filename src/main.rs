use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("Github profile name is: {:?}", args[1]);
}
