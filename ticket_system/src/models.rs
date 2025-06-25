use std::collections::{HashMap, HashSet};
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Venue {
    name: String,
    zones: HashMap<String, Zone>, // zone name -> Zone
    max_capacity: usize,
}

#[derive(Debug)]
pub struct Zone {
    name: String,
    seat_count: usize,
    price: f32,
}

#[derive(Debug)]
pub struct Show {
    id: usize,
    title: String,
    date: NaiveDate,
    venue: Venue,
    tickets_sold: HashMap<String, HashSet<usize>>, // zone name -> seat numbers sold
}

#[derive(Debug)]
pub struct Ticket {
    show_id: usize,
    zone: String,
    seat_number: usize,
    buyer_name: String,
}