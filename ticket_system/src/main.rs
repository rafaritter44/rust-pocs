mod models;
mod system;

use crate::system::TicketSystem;
use chrono::NaiveDate;

fn main() {
    let mut system = TicketSystem::new();
    let venue = models::Venue {
        name: "Concert Hall".into(),
        zones: vec![
            ("VIP".into(), models::Zone { name: "VIP".into(), seat_count: 50, price: 100.0 }),
            ("General".into(), models::Zone { name: "General".into(), seat_count: 200, price: 50.0 }),
        ].into_iter().collect(),
        max_capacity: 250,
    };
    let show_id = system.add_show(
        "Classical Concert".into(),
        NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
        venue
    );

    system.buy_ticket(show_id, "VIP".into(), 1, "Alice".into()).unwrap();
    system.buy_ticket(show_id, "General".into(), 10, "Bob".into()).unwrap();
    system.buy_ticket(show_id, "General".into(), 2, "Charlie".into()).unwrap();

    let shows = system.list_shows();
    for show in shows {
        println!("Show ID: {}, Title: {}, Date: {}", show.id, show.title, show.date);
        for (zone_name, sold_seats) in &show.tickets_sold {
            println!("  Zone: {}, Seats Sold: {:?}", zone_name, sold_seats);
        }
    }
}