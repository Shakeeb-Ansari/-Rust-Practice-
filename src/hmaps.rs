use std::collections::HashMap;

pub fn run() {
    
     let mut d = HashMap::new();
     d.insert(String::from("Car"), 200u8);
    
        //OR we can make hashmap through vector like this:

     let k = vec!["Blue","Yellow"];
     let l = vec![10,20];
     let mut data : HashMap<_,_> = 
     k.into_iter().zip(l.into_iter()).collect();
     data.insert("Green", 30);
     println!("Hashmap <k,v>: {:?}", data);

    
    // Accessing Values in a Hashmap:
     let mut scores = HashMap::new();
     scores.insert(String::from("Blue"), 10);
     scores.insert(String::from("Yellow"), 50);
     let team_name = String::from("Blue");
     let score = scores.get(&team_name);


    // Iterating over each key,value pair using for loop: 
        
     let mut scores = HashMap::new();
    
     scores.insert(String::from("Blue"), 10);
     scores.insert(String::from("Yellow"), 50);
    
     for (key, value) in &scores {
         println!("{}: {}", key, value);
     }

    
    // Updating or Overwriting a value :
     let mut scores = HashMap::new();

     scores.insert(String::from("Blue"), 10);
     scores.insert(String::from("Blue"), 25);

     println!("{:?}", scores);    //This code will print {"Blue": 25}. The original value of 10 has been overwritten.

    
    // Only inserting the value if the key has no value:

     let mut scores = HashMap::new();
     scores.insert(String::from("Blue"), 10);

     scores.entry(String::from("Yellow")).or_insert(50);
     scores.entry(String::from("Blue")).or_insert(50);

     println!("{:?}", scores);    // will print {"Yellow": 50, "Blue": 10}.The second call to entry will not change the hash map because the Blue team already has the value 10.


    //Updating a value based on the old value:
    // Following is a code that counts how many times each word appears in some text.
     let text = "hello world wonderful world";

     let mut map = HashMap::new();

     for word in text.split_whitespace() {
         let count = map.entry(word).or_insert(0);
         *count += 1;
     }

     println!("{:?}", map); //This code will print {"world": 2, "hello": 1, "wonderful": 1}
}