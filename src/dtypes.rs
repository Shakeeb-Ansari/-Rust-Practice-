pub fn run() {

    // Int,float,explicit:
    let x = 1;      // By default int is assigned to i32 (32-bit signed integer)
    let y = 2.5;    // By default float is assigned to f64
    let z: i8 = 20;     // Explicit type, we specified the int type ourselves i.e. i8

    println!("x: {}, y: {}, z: {}", x, y, z);

    // Boolean:
    let active: bool = true;
    println!("{:?}", active);

    let is_lesser = 3<5;      // Boolean from expression
    println!("{:?}", is_lesser);

    // Unicode char:
    let x = 's';
    println!("{:?}", x);

    // Emoji through unicode:
    let sleep_face = '\u{1F634}' ;
    println!("{:?}", sleep_face);

    // Strings:

    let mut q= String::from("Open ");
    println!("Length is {}",q.len());       //To print the length of string

    q.push('S');               //Pushing char W into 'Hello '
    println!("{}",q);

    q.push_str("ource");    //Pushing string 'ource' so it becomes Source
    println!("{}",q);

    println!("Replace: {}",q.replace("Source","Resource"));     //Source will be replaced by Resource

    let mut h = String::with_capacity(5);   // String can also be made through capacity method
    h.push_str("Rive");
    h.push('r');
    println!("{}", h);






}
