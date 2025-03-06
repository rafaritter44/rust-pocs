fn main() {
    let x = 5;
    let x = x + 1;

    let mut y = 50;
    y = y + 10;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // The value of x in the inner scope is: 12

        y = y * 20;
        println!("The value of y in the inner scope is: {y}"); // The value of y in the inner scope is: 1200
    }

    println!("The value of x is: {x}"); // The value of x is: 6
    println!("The value of y is: {y}"); // The value of y is: 1200

    let x = format!("{x} satoshi");
    println!("The new value of x is {x}") // The new value of x is 6 satoshi
}