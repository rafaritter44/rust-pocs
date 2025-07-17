mod models;
mod system;

use std::{sync::{Arc, Mutex}, thread};

use crate::system::TicketSystem;
use chrono::NaiveDate;

fn main() {
    let system = Arc::new(Mutex::new(TicketSystem::new()));
    let venue = models::Venue {
        name: "Concert Hall".into(),
        zones: vec![
            ("VIP".into(), models::Zone { name: "VIP".into(), seat_count: 50, price: 100.0 }),
            ("General".into(), models::Zone { name: "General".into(), seat_count: 200, price: 50.0 }),
        ].into_iter().collect(),
        max_capacity: 250,
    };
    let show_id = system.lock().unwrap().add_show(
        "Classical Concert".into(),
        NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
        venue
    );

    let threads = vec![
        ("VIP", 1, "Alice"),
        ("General", 10, "Bob"),
        ("General", 2, "Charlie"),
    ]
    .into_iter()
    .map(|(zone, seat, buyer)| {
        let system_clone = Arc::clone(&system);
        thread::spawn(move || {
            system_clone.lock().unwrap().buy_ticket(show_id, zone.into(), seat, buyer.into()).unwrap();
        })
    })
    .collect::<Vec<_>>();

    for t in threads {
        t.join().unwrap();
    }

    let system = system.lock().unwrap();
    let shows = system.list_shows();
    for show in shows {
        println!("Show ID: {}, Title: {}, Date: {}", show.id, show.title, show.date);
        for (zone_name, sold_seats) in &show.tickets_sold {
            println!("  Zone: {}, Seats Sold: {:?}", zone_name, sold_seats);
        }
    }
}