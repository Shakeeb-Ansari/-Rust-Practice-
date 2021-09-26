pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person = true;

    if age>=20 && check_id || knows_person {                    //&&=and , ||=or
        println!("Bartender: What would you like to drink?");
    }   else if age < 20 && check_id {
        println!("Bartender: Sorry, you are not welcomed here.");
    }   else {
        println!("Bartender: Please show me your ID");
    }

    let is_of_age = if age >= 20 {true} else {false};       //Shorthand If
    println!("Is of Age: {}", is_of_age);


}