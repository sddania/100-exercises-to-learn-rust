use rocket::serde::Deserialize;
use crate::domain::data::Status;
use crate::domain::{description::TicketDescription, title::TicketTitle};
use crate::dto::ticket_update::TicketUpdate;
use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct UpdateTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateTicketError {
    #[error("Title validation failed: {0}")]
    InvalidTitle(String),
    #[error("Description validation failed: {0}")]
    InvalidDescription(String),
    #[error("Invalid status: {0}")]
    InvalidStatus(String),
}

impl TryFrom<UpdateTicketRequest> for TicketUpdate {
    type Error = UpdateTicketError;
    
    fn try_from(request: UpdateTicketRequest) -> Result<Self, Self::Error> {
        let title = if let Some(title_str) = request.title {
            Some(TicketTitle::try_from(title_str.as_str())
                .map_err(|e| UpdateTicketError::InvalidTitle(e.to_string()))?)
        } else {
            None
        };
        
        let description = if let Some(desc_str) = request.description {
            Some(TicketDescription::try_from(desc_str.as_str())
                .map_err(|e| UpdateTicketError::InvalidDescription(e.to_string()))?)
        } else {
            None
        };
        
        let status = if let Some(status_str) = request.status {
            let status = match status_str.as_str() {
                "ToDo" => Status::ToDo,
                "InProgress" => Status::InProgress,
                "Done" => Status::Done,
                _ => return Err(UpdateTicketError::InvalidStatus(status_str)),
            };
            Some(status)
        } else {
            None
        };
        
        Ok(TicketUpdate {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_update_ticket_request_try_from_valid() {
        let request = UpdateTicketRequest {
            title: Some("New Title".to_string()),
            description: Some("New Description".to_string()),
            status: Some("InProgress".to_string()),
        };

        let result = TicketUpdate::try_from(request);
        assert!(result.is_ok());
        
        let update = result.unwrap();
        assert!(update.title.is_some());
        assert!(update.description.is_some());
        assert_eq!(update.status, Some(Status::InProgress));
    }

    #[test]
    fn test_update_ticket_request_try_from_partial() {
        let request = UpdateTicketRequest {
            title: Some("Only Title".to_string()),
            description: None,
            status: None,
        };

        let result = TicketUpdate::try_from(request);
        assert!(result.is_ok());
        
        let update = result.unwrap();
        assert!(update.title.is_some());
        assert!(update.description.is_none());
        assert!(update.status.is_none());
    }

    #[test]
    fn test_update_ticket_request_try_from_invalid_status() {
        let request = UpdateTicketRequest {
            title: None,
            description: None,
            status: Some("InvalidStatus".to_string()),
        };

        let result = TicketUpdate::try_from(request);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_ticket_request_try_from_empty() {
        let request = UpdateTicketRequest {
            title: None,
            description: None,
            status: None,
        };

        let result = TicketUpdate::try_from(request);
        assert!(result.is_ok());
        
        let update = result.unwrap();
        assert!(update.title.is_none());
        assert!(update.description.is_none());
        assert!(update.status.is_none());
    }

    #[test]
    fn test_update_ticket_request_try_from_invalid_title() {
        let request = UpdateTicketRequest {
            title: Some("".to_string()), // Empty title should fail
            description: None,
            status: None,
        };

        let result = TicketUpdate::try_from(request);
        assert!(result.is_err());
    }
}