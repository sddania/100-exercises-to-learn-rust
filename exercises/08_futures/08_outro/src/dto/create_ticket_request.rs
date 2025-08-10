use rocket::serde::Deserialize;
use crate::domain::data::TicketDraft;
use crate::domain::{description::TicketDescription, title::TicketTitle};
use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateTicketError {
    #[error("Title validation failed: {0}")]
    InvalidTitle(String),
    #[error("Description validation failed: {0}")]
    InvalidDescription(String),
}

impl TryFrom<CreateTicketRequest> for TicketDraft {
    type Error = CreateTicketError;
    
    fn try_from(request: CreateTicketRequest) -> Result<Self, Self::Error> {
        let title = TicketTitle::try_from(request.title.as_str())
            .map_err(|e| CreateTicketError::InvalidTitle(e.to_string()))?;
            
        let description = TicketDescription::try_from(request.description.as_str())
            .map_err(|e| CreateTicketError::InvalidDescription(e.to_string()))?;
            
        Ok(TicketDraft { title, description })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_create_ticket_request_try_from_valid() {
        let request = CreateTicketRequest {
            title: "Test Title".to_string(),
            description: "Test Description".to_string(),
        };

        let result = TicketDraft::try_from(request);
        assert!(result.is_ok());
        
        let draft = result.unwrap();
        assert_eq!(draft.title.value(), "Test Title");
        assert_eq!(draft.description.value(), "Test Description");
    }

    #[test]
    fn test_create_ticket_request_try_from_invalid_title() {
        let request = CreateTicketRequest {
            title: "".to_string(), // Empty title should fail
            description: "Test Description".to_string(),
        };

        let result = TicketDraft::try_from(request);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_ticket_request_try_from_invalid_description() {
        let request = CreateTicketRequest {
            title: "Valid Title".to_string(),
            description: "".to_string(), // Empty description should fail
        };

        let result = TicketDraft::try_from(request);
        assert!(result.is_err());
    }
}