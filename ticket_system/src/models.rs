use std::collections::{HashMap, HashSet};
use chrono::NaiveDate;

pub struct Venue {
    pub name: String,
    pub zones: HashMap<String, Zone>, // zone name -> Zone
    pub max_capacity: usize,
}

pub struct Zone {
    pub name: String,
    pub seat_count: usize,
    pub price: f32,
}

pub struct Show {
    pub id: usize,
    pub title: String,
    pub date: NaiveDate,
    pub venue: Venue,
    pub tickets_sold: HashMap<String, HashSet<usize>>, // zone name -> seat numbers sold
}

pub struct Ticket {
    pub show_id: usize,
    pub zone: String,
    pub seat_number: usize,
    pub buyer_name: String,
}