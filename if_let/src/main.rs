fn print_max(config_max: Option<u8>) {
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}.");
    } else {
        println!("No maximum is configured.");
    }
}

fn main() {
    print_max(Some(3u8));
    print_max(None);
}
