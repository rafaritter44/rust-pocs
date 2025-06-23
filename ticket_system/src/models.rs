use std::collections::{HashMap, HashSet};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
struct Venue {
    name: String,
    zones: HashMap<String, Zone>, // zone name -> Zone
    max_capacity: usize,
}

#[derive(Debug)]
struct Zone {
    name: String,
    seat_count: usize,
    price: f32,
}

#[derive(Debug)]
struct Show {
    id: usize,
    title: String,
    date: NaiveDate,
    venue: Venue,
    tickets_sold: HashMap<String, HashSet<usize>>, // zone name -> seat numbers sold
}

#[derive(Debug)]
struct Ticket {
    show_id: usize,
    zone: String,
    seat_number: usize,
    buyer_name: String,
}