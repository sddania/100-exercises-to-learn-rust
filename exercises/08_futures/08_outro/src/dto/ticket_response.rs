use rocket::serde::Serialize;
use crate::domain::data::{Ticket, Status};

#[derive(Serialize)]
pub struct TicketResponse {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: &'static str,
}

impl From<&Ticket> for TicketResponse {
    fn from(ticket: &Ticket) -> Self {
        TicketResponse {
            id: ticket.id.value(),
            title: ticket.title.value().to_string(),
            description: ticket.description.value().to_string(),
            status: match ticket.status {
                Status::ToDo => "ToDo",
                Status::InProgress => "InProgress",
                Status::Done => "Done",
            },
        }
    }
}

impl From<Ticket> for TicketResponse {
    fn from(ticket: Ticket) -> Self {
        TicketResponse::from(&ticket)
    }
}

impl From<&mut Ticket> for TicketResponse {
    fn from(ticket: &mut Ticket) -> Self {
        TicketResponse::from(&*ticket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::data::{Status, Ticket};
    use crate::domain::store::TicketId;
    use crate::domain::{title::TicketTitle, description::TicketDescription};
    use std::convert::TryFrom;

    #[test]
    fn test_ticket_response_from_ticket_ref() {
        let ticket = Ticket {
            id: TicketId::new(1),
            title: TicketTitle::try_from("Test Title").unwrap(),
            description: TicketDescription::try_from("Test Description").unwrap(),
            status: Status::InProgress,
        };

        let response = TicketResponse::from(&ticket);
        
        assert_eq!(response.id, 1);
        assert_eq!(response.title, "Test Title");
        assert_eq!(response.description, "Test Description");
        assert_eq!(response.status, "InProgress");
    }

    #[test]
    fn test_ticket_response_from_owned_ticket() {
        let ticket = Ticket {
            id: TicketId::new(42),
            title: TicketTitle::try_from("Owned Title").unwrap(),
            description: TicketDescription::try_from("Owned Description").unwrap(),
            status: Status::Done,
        };

        let response = TicketResponse::from(ticket);
        
        assert_eq!(response.id, 42);
        assert_eq!(response.title, "Owned Title");
        assert_eq!(response.description, "Owned Description");
        assert_eq!(response.status, "Done");
    }

    #[test]
    fn test_ticket_response_from_mut_ticket() {
        let mut ticket = Ticket {
            id: TicketId::new(99),
            title: TicketTitle::try_from("Mutable Title").unwrap(),
            description: TicketDescription::try_from("Mutable Description").unwrap(),
            status: Status::ToDo,
        };

        let response = TicketResponse::from(&mut ticket);
        
        assert_eq!(response.id, 99);
        assert_eq!(response.title, "Mutable Title");
        assert_eq!(response.description, "Mutable Description");
        assert_eq!(response.status, "ToDo");
    }

    #[test]
    fn test_ticket_response_status_mapping() {
        let test_cases = vec![
            (Status::ToDo, "ToDo"),
            (Status::InProgress, "InProgress"),
            (Status::Done, "Done"),
        ];

        for (status, expected_str) in test_cases {
            let ticket = Ticket {
                id: TicketId::new(1),
                title: TicketTitle::try_from("Title").unwrap(),
                description: TicketDescription::try_from("Description").unwrap(),
                status,
            };

            let response = TicketResponse::from(&ticket);
            assert_eq!(response.status, expected_str);
        }
    }
}