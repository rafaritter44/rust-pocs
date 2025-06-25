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
}