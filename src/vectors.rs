pub fn run() {
    let mut vect = vec![3,6,9,12,15,18];
    println!("{:?}", vect);

    println!("Value at 2nd index: {}", vect[2]);    //Printing a specific i.e 2nd index only

    vect[3] = 13;
    println!("Modified value of 3rd index: {:?}", vect);  //Re-assign a value at specific index

    vect.push(20);         // New values can be pushed. Here '20' will be added after '18' in our vector
    println!("Updated vector: {:?}", vect);

    let slice: &[i32] = &vect[0..3];    // Slicing off a vector
    println!("Vector slice: {:?}", slice);

    for i in vect.iter() {                 // Looping through a Vector
        println!("Vector Value: {}", i);
    }

    for i in vect.iter_mut() {
        *i *=5;    // Multiplying all vec values by 5
    }
    println!("Multiplied Vec Value: {:?}", vect);

}