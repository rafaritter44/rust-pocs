use chrono::NaiveDate;
use dashmap::{DashMap, DashSet};

#[derive(Clone)]
pub struct Venue {
    pub name: String,
    pub zones: DashMap<String, Zone>, // zone name -> Zone
    pub max_capacity: usize,
}

#[derive(Clone)]
pub struct Zone {
    pub name: String,
    pub seat_count: usize,
    pub price: f32,
}

#[derive(Clone)]
pub struct Show {
    pub id: usize,
    pub title: String,
    pub date: NaiveDate,
    pub venue: Venue,
    pub tickets_sold: DashMap<String, DashSet<usize>>, // zone name -> seat numbers sold
}

pub struct Ticket {
    pub show_id: usize,
    pub zone: String,
    pub seat_number: usize,
    pub buyer_name: String,
}