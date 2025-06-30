use std::collections::{HashMap, HashSet};
use chrono::NaiveDate;
use crate::models::{Venue, Zone, Show, Ticket};

pub struct TicketSystem {
    shows: HashMap<usize, Show>,
    next_show_id: usize,
}

impl TicketSystem {
    pub fn new() -> Self {
        TicketSystem {
            shows: HashMap::new(),
            next_show_id: 1,
        }
    }
    pub fn add_show(&mut self, title: String, date: NaiveDate, venue: Venue) -> usize {
        let show_id = self.next_show_id;
        let show = Show {
            id: show_id,
            title,
            date,
            venue,
            tickets_sold: HashMap::new(),
        };
        self.shows.insert(show_id, show);
        self.next_show_id += 1;
        show_id
    }
    pub fn buy_ticket(&mut self, show_id: usize, zone: String, seat_number: usize, buyer_name: String) -> Result<Ticket, String> {
        if let Some(show) = self.shows.get_mut(&show_id) {
            if let Some(zone_info) = show.venue.zones.get(&zone) {
                if seat_number == 0 || seat_number > zone_info.seat_count {
                    return Err("Invalid seat number".to_string());
                }
                let sold_seats = show.tickets_sold.entry(zone.clone()).or_insert_with(HashSet::new);
                if sold_seats.contains(&seat_number) {
                    return Err("Seat already sold".to_string());
                }
                sold_seats.insert(seat_number);
                Ok(Ticket {
                    show_id,
                    zone,
                    seat_number,
                    buyer_name,
                })
            } else {
                Err("Zone not found".to_string())
            }
        } else {
            Err("Show not found".to_string())
        }
    }
    pub fn list_shows(&self) -> Vec<&Show> {
        self.shows.values().collect()
    }
}