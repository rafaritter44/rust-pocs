fn main() {
    // unwrap_or
    assert_eq!(Some(5).unwrap_or(-1), 5);
    assert_eq!(None.unwrap_or(-1), -1);

    // unwrap_or_else
    assert_eq!(Some(5).unwrap_or_else(|| 10 * 10), 5);
    assert_eq!(None.unwrap_or_else(|| 10 * 10), 100);

    // unwrap_or_default
    assert_eq!(Some(5).unwrap_or_default(), 5);
    assert_eq!(None::<i32>.unwrap_or_default(), 0);

    // map_or
    assert_eq!(Some(5).map_or(String::from("NaN"), |n| n.to_string()), "5");
    assert_eq!(None::<i32>.map_or(String::from("NaN"), |n| n.to_string()), "NaN");

    // ok_or
    assert_eq!(Some(5).ok_or("NaN"), Ok(5));
    assert_eq!(None::<i32>.ok_or("NaN"), Err("NaN"));

    // and_then
    assert_eq!(Some(5).and_then(|n| Some(n.to_string())), Some(String::from("5")));
    assert_eq!(None::<i32>.and_then(|n| Some(n.to_string())), None);

    // filter
    assert_eq!(Some(5).filter(|&n| n < 0), None);
    assert_eq!(None::<i32>.filter(|&n| n < 0), None);
}
