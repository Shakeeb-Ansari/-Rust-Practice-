pub fn run() {
    let mut sage = 0;

    loop {                             // Infinite Loop
        sage += 2;
        println!("Number: {}", sage);
    
        if sage == 14 {                //Keeping the sage '14' so that it doesn't loop forever
            break;
        }
    }

    while sage <= 50 {                 //While loop 
        if sage % 15 == 0 {
            println!("FizzBuzz");
        }else if sage % 3 == 0 {
            println!("Fizz");
        }else if sage % 5 == 0 {
            println!("Buzz");
        }else {
            println!("{}", sage);
        }
        
        sage += 1;
    }

    for i in 0..50 {               // For range
         if i % 15 == 0 {
              println!("FizzBuzz");
          }else if i % 3 == 0 {
              println!("Fizz");
          }else if i % 5 == 0 {
              println!("Buzz");
          }else {
              println!("{}", i);
          }      
      }
      
    
    

}