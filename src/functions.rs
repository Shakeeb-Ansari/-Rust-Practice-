pub fn run() {
    greeting("Salute", "Ezio Auditore");    // Function is being called here

    let return_sub = sub(10,5);
    println!("Subtraction: {}", return_sub);

    let num3: i32 = 15;                         // Closures
    let num4: i32 = 20;
    let add_nums = |num1: i32, num2: i32| num1 + num2 + num3 + num4;
    println!("Closure Sum: {}", add_nums(6,6));

}

    

fn greeting(greet: &str, name: &str) {
    println!("{} {}. How's it going, fratello mio ", greet, name);
}

fn sub(n1: i32, n2: i32) -> i32 {
    n1-n2
} 
