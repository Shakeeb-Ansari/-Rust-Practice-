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


impl Shape {                // Associated or related function
    fn new(width: u16, height: u16) -> Shape {
        Shape {
            width,
            height,
        }
    }
}

pub fn run() {

    let h = Shape::new(30,40);        // For associated function

    h.display();
}