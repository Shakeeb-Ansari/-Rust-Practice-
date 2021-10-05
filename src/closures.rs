pub fn run() {
    let num3: i32 = 15;
    let add_nums = |num1: i32, num2: i32| num1 + num2 + num3;
    println!("Closure Sum: {}", add_nums(6,6));

    // Another example:
    
    // Increment via closures and functions.
    fn function(i: i32) -> i32 { i + 2}
    
    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // These nameless functions are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 4 };
    let closure_inferred  = |i     |          i + 6  ;
    
    let i = 5;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    
    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
    
    }
    
