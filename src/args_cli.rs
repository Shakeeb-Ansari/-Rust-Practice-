use std::env;

pub fn run() {
    let arg : Vec<String> = env::args().collect();
    println!("{:?}", arg); 

    let x = &arg[1];    //Note that the project's path is on the 0th index by default
    let y = &arg[2];    // So the indexes are taken after the 0th one

    println!("Launching: {}", x);
    println!("and {}", y);
}