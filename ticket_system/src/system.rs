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
}