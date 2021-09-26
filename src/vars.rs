pub fn run() {

    // Variables: 
    let name = "Shakeeb";
    let mut age = 20;       // Creating a mutable variable
    println!("My name is {} and I am {}", name, age);

    age = 21;
    println!("My name is {} and I am {}", name, age);

    // Assigning multiple Variables:
    let (my_name , my_age, profession)=("Shakeeb", 20, "Student");
    println!("Hi, it's {}. I am {} and I am a {}", my_name, my_age, profession);

    // Defining Constants:
    const ID: i32 = 100;
    println!("Your ID is {}", ID);
}
           