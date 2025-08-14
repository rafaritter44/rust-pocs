use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::NaiveDate;
use dashmap::{DashMap, DashSet};
use crate::models::{Venue, Show, Ticket};

pub struct TicketSystem {
    shows: DashMap<usize, Show>,
    next_show_id: AtomicUsize,
}

impl TicketSystem {
    pub fn new() -> Self {
        TicketSystem {
            shows: DashMap::new(),
            next_show_id: AtomicUsize::new(1),
        }
    }
    pub fn add_show(&self, title: String, date: NaiveDate, venue: Venue) -> usize {
        let show_id = self.next_show_id.fetch_add(1, Ordering::SeqCst);
        let show = Show {
            id: show_id,
            title,
            date,
            venue,
            tickets_sold: DashMap::new(),
        };
        self.shows.insert(show_id, show);
        show_id
    }
    pub fn buy_ticket(&self, show_id: usize, zone: String, seat_number: usize, buyer_name: String) -> Result<Ticket, String> {
        if let Some(show) = self.shows.get_mut(&show_id) {
            if let Some(zone_info) = show.venue.zones.get(&zone) {
                if seat_number == 0 || seat_number > zone_info.seat_count {
                    return Err("Invalid seat number".into());
                }
                let sold_seats = show.tickets_sold.entry(zone.clone()).or_insert_with(DashSet::new);
                if sold_seats.contains(&seat_number) {
                    return Err("Seat already sold".into());
                }
                sold_seats.insert(seat_number);
                Ok(Ticket {
                    show_id,
                    zone,
                    seat_number,
                    buyer_name,
                })
            } else {
                Err("Zone not found".into())
            }
        } else {
            Err("Show not found".into())
        }
    }
    pub fn list_shows(&self) -> Vec<Show> {
        self.shows.iter().map(|entry| entry.value().clone()).collect()
    }
}