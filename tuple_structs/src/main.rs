struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    println!("Black = ({}, {}, {})", black.0, black.1, black.2);

    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;
    println!("Origin = ({}, {}, {})", x, y, z);
}