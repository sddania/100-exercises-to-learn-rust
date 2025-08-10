use crate::domain::data::TicketDraft;
use crate::domain::store::{TicketId, TicketStore};
use crate::dto::{TicketResponse, CreateTicketRequest, UpdateTicketRequest, TicketUpdate};
use rocket::{get, post, put, State, serde::json::Json};
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

#[get("/ticket/<id>")]
pub fn index(id: u64, store: &State<Arc<Mutex<TicketStore>>>) -> Result<Json<TicketResponse>, rocket::http::Status> {
    let store = store.lock().unwrap();
    let ticket_id = TicketId::new(id);
    
    match store.get(ticket_id) {
        Some(ticket) => {
            let response = TicketResponse::from(ticket);
            Ok(Json(response))
        }
        None => Err(rocket::http::Status::NotFound)
    }
}

#[post("/ticket", data = "<request>")]
pub fn save(request: Json<CreateTicketRequest>, store: &State<Arc<Mutex<TicketStore>>>) -> Result<Json<TicketResponse>, rocket::http::Status> {
    let draft = match TicketDraft::try_from(request.into_inner()) {
        Ok(draft) => draft,
        Err(_) => return Err(rocket::http::Status::BadRequest),
    };
    
    let mut store = store.lock().unwrap();
    let ticket_id = store.add_ticket(draft);
    
    let ticket = store.get(ticket_id).unwrap();
    
    let response = TicketResponse::from(ticket);
    
    Ok(Json(response))
}

#[put("/ticket/<id>", data = "<request>")]
pub fn change(id: u64, request: Json<UpdateTicketRequest>, store: &State<Arc<Mutex<TicketStore>>>) -> Result<Json<TicketResponse>, rocket::http::Status> {
    let update = match TicketUpdate::try_from(request.into_inner()) {
        Ok(update) => update,
        Err(_) => return Err(rocket::http::Status::BadRequest),
    };
    
    let mut store = store.lock().unwrap();
    let ticket_id = TicketId::new(id);
    
    let ticket = match store.get_mut(ticket_id) {
        Some(ticket) => ticket,
        None => return Err(rocket::http::Status::NotFound),
    };
    
    if let Some(title) = update.title {
        ticket.title = title;
    }
    
    if let Some(description) = update.description {
        ticket.description = description;
    }
    
    if let Some(status) = update.status {
        ticket.status = status;
    }
    
    let response = TicketResponse::from(ticket);
    
    Ok(Json(response))
}

