struct Shape {
    width : u16,            // Traditional Struct
    height : u16,
}

impl Shape {
    fn area(&self) -> u16 {         // Method which returns the values rather than printing 
        self.height * self.width
    }

    fn display(&self) {
        println!("For given width: {} and height: {}, area is {}", self.width, self.height, self.area());
    }
    
}

pub fn run() {
    let z = Shape {
        width: 20,
        height: 30,
    };

    z.display();

}