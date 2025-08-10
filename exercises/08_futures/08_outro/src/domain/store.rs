use crate::domain::data::{Status, Ticket, TicketDraft};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);


impl TicketId {
    pub fn new(id: u64) -> Self {
        TicketId(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    // The `get` method should return a handle to the ticket
    // which allows the caller to either read or modify the ticket.
    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }
    
    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }

    pub fn update_ticket(&mut self, id: TicketId, ticket: Ticket) -> Result<(), &str> {
        if self.tickets.contains_key(&id) {
            self.tickets.insert(id, ticket);
            Ok(())
        } else {
            Err("The ticket does not exist")
        }
    }
}
