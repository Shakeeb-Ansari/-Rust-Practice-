pub fn run() {
    let mut arr =[10u8,11,12,13,14,15];
    println!("{:?}",arr);

    println!("Value at 3rd index is {}", arr[3]);  // Printing a specific i.e 3rd index only

    arr[1] = 25;        //Replacing or modifying specific index's value
    println!("The changed value at 1st index is {:?}", arr);

    let slice: &[i8] = &arr[1..5];
    println!("Slice: {:?}", slice);

}