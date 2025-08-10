use crate::domain::data::Status;
use crate::domain::{description::TicketDescription, title::TicketTitle};

#[derive(Debug, Clone)]
pub struct TicketUpdate {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>, 
    pub status: Option<Status>,
}