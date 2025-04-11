struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_area(&self) {
        println!(
            "The area of the rectangle is {} square pixels.",
            self.width * self.height
        );
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    let rect2 = Rectangle::new(45, 25);
    let rect3 = Rectangle::square(40);
    
    rect1.print_area();
    rect2.print_area();
    rect3.print_area();

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}