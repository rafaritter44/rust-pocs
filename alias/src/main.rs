use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

fn main() {
    println!("{:?}", function1());
    println!("{:?}", function2());
}