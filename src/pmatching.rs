pub fn run() {
     let b= 1;
     match b {
         1 => println!("1 it is, matched!"),
         _ => println!("Kuch bhi"),


     }
    
     let m=5;
     match m {
         4 => println!("4"),
         5 | 7 => println!("You Either entered 5 Or 7"),
         _ =>println!("Kuch bhi"),
     }


     enum Currency {
         Coins,
         Cash,
         Crypto,
     }
     fn validity(var: Currency) -> u8 {
         match var  {
             Currency::Coins => 5,
             Currency::Cash => 10,
             Currency::Crypto => 15,
     }
     }
}