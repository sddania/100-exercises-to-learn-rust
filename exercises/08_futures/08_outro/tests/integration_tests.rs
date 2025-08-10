use outro_08::domain::store::TicketStore;
use outro_08::routes::ticket::{index, save, change};
use rocket::local::asynchronous::Client;
use rocket::{Build, Rocket, routes};
use rocket::http::{ContentType, Status};
use std::sync::{Arc, Mutex};

async fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(Arc::new(Mutex::new(TicketStore::new())))
        .mount("/api", routes![index, save, change])
}

#[tokio::test]
async fn test_get_ticket_success() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    // First create a ticket
    let create_response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "Test Title", "description": "Test Description"}"#)
        .dispatch()
        .await;

    assert_eq!(create_response.status(), Status::Ok);
    let created_ticket: serde_json::Value = create_response.into_json().await.expect("valid json");
    let ticket_id = created_ticket["id"].as_u64().expect("id should be u64");

    // Then retrieve it
    let get_response = client
        .get(format!("/api/ticket/{}", ticket_id))
        .dispatch()
        .await;

    assert_eq!(get_response.status(), Status::Ok);
    let retrieved_ticket: serde_json::Value = get_response.into_json().await.expect("valid json");
    
    assert_eq!(retrieved_ticket["id"], ticket_id);
    assert_eq!(retrieved_ticket["title"], "Test Title");
    assert_eq!(retrieved_ticket["description"], "Test Description");
    assert_eq!(retrieved_ticket["status"], "ToDo");
}

#[tokio::test]
async fn test_get_ticket_not_found() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    let response = client
        .get("/api/ticket/999")
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
}

#[tokio::test]
async fn test_create_ticket_success() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    let response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "New Ticket", "description": "New Description"}"#)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let created_ticket: serde_json::Value = response.into_json().await.expect("valid json");
    
    assert!(created_ticket["id"].as_u64().is_some());
    assert_eq!(created_ticket["title"], "New Ticket");
    assert_eq!(created_ticket["description"], "New Description");
    assert_eq!(created_ticket["status"], "ToDo");
}

#[tokio::test]
async fn test_create_ticket_invalid_title() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    let response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "", "description": "Valid Description"}"#)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_ticket_invalid_description() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    let response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "Valid Title", "description": ""}"#)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_create_ticket_malformed_json() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    let response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "Valid Title""#) // Missing closing brace
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_update_ticket_success() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    // First create a ticket
    let create_response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "Original Title", "description": "Original Description"}"#)
        .dispatch()
        .await;

    let created_ticket: serde_json::Value = create_response.into_json().await.expect("valid json");
    let ticket_id = created_ticket["id"].as_u64().expect("id should be u64");

    // Then update it
    let update_response = client
        .put(format!("/api/ticket/{}", ticket_id))
        .header(ContentType::JSON)
        .body(r#"{"title": "Updated Title", "description": "Updated Description", "status": "InProgress"}"#)
        .dispatch()
        .await;

    assert_eq!(update_response.status(), Status::Ok);
    let updated_ticket: serde_json::Value = update_response.into_json().await.expect("valid json");
    
    assert_eq!(updated_ticket["id"], ticket_id);
    assert_eq!(updated_ticket["title"], "Updated Title");
    assert_eq!(updated_ticket["description"], "Updated Description");
    assert_eq!(updated_ticket["status"], "InProgress");
}

#[tokio::test]
async fn test_update_ticket_partial() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    // First create a ticket
    let create_response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "Original Title", "description": "Original Description"}"#)
        .dispatch()
        .await;

    let created_ticket: serde_json::Value = create_response.into_json().await.expect("valid json");
    let ticket_id = created_ticket["id"].as_u64().expect("id should be u64");

    // Update only the status
    let update_response = client
        .put(format!("/api/ticket/{}", ticket_id))
        .header(ContentType::JSON)
        .body(r#"{"status": "Done"}"#)
        .dispatch()
        .await;

    assert_eq!(update_response.status(), Status::Ok);
    let updated_ticket: serde_json::Value = update_response.into_json().await.expect("valid json");
    
    assert_eq!(updated_ticket["id"], ticket_id);
    assert_eq!(updated_ticket["title"], "Original Title"); // Unchanged
    assert_eq!(updated_ticket["description"], "Original Description"); // Unchanged
    assert_eq!(updated_ticket["status"], "Done"); // Changed
}

#[tokio::test]
async fn test_update_ticket_not_found() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    let response = client
        .put("/api/ticket/999")
        .header(ContentType::JSON)
        .body(r#"{"title": "Updated Title"}"#)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
}

#[tokio::test]
async fn test_update_ticket_invalid_status() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    // First create a ticket
    let create_response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "Test Title", "description": "Test Description"}"#)
        .dispatch()
        .await;

    let created_ticket: serde_json::Value = create_response.into_json().await.expect("valid json");
    let ticket_id = created_ticket["id"].as_u64().expect("id should be u64");

    // Try to update with invalid status
    let update_response = client
        .put(format!("/api/ticket/{}", ticket_id))
        .header(ContentType::JSON)
        .body(r#"{"status": "InvalidStatus"}"#)
        .dispatch()
        .await;

    assert_eq!(update_response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_update_ticket_invalid_title() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    // First create a ticket
    let create_response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "Test Title", "description": "Test Description"}"#)
        .dispatch()
        .await;

    let created_ticket: serde_json::Value = create_response.into_json().await.expect("valid json");
    let ticket_id = created_ticket["id"].as_u64().expect("id should be u64");

    // Try to update with empty title
    let update_response = client
        .put(format!("/api/ticket/{}", ticket_id))
        .header(ContentType::JSON)
        .body(r#"{"title": ""}"#)
        .dispatch()
        .await;

    assert_eq!(update_response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_complete_workflow() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    
    // 1. Create a ticket
    let create_response = client
        .post("/api/ticket")
        .header(ContentType::JSON)
        .body(r#"{"title": "Workflow Test", "description": "Testing complete workflow"}"#)
        .dispatch()
        .await;

    assert_eq!(create_response.status(), Status::Ok);
    let created_ticket: serde_json::Value = create_response.into_json().await.expect("valid json");
    let ticket_id = created_ticket["id"].as_u64().expect("id should be u64");
    assert_eq!(created_ticket["status"], "ToDo");

    // 2. Move to InProgress
    let update1_response = client
        .put(format!("/api/ticket/{}", ticket_id))
        .header(ContentType::JSON)
        .body(r#"{"status": "InProgress"}"#)
        .dispatch()
        .await;

    assert_eq!(update1_response.status(), Status::Ok);
    let updated_ticket1: serde_json::Value = update1_response.into_json().await.expect("valid json");
    assert_eq!(updated_ticket1["status"], "InProgress");

    // 3. Complete the ticket
    let update2_response = client
        .put(format!("/api/ticket/{}", ticket_id))
        .header(ContentType::JSON)
        .body(r#"{"status": "Done"}"#)
        .dispatch()
        .await;

    assert_eq!(update2_response.status(), Status::Ok);
    let updated_ticket2: serde_json::Value = update2_response.into_json().await.expect("valid json");
    assert_eq!(updated_ticket2["status"], "Done");

    // 4. Verify final state
    let get_response = client
        .get(format!("/api/ticket/{}", ticket_id))
        .dispatch()
        .await;

    assert_eq!(get_response.status(), Status::Ok);
    let final_ticket: serde_json::Value = get_response.into_json().await.expect("valid json");
    assert_eq!(final_ticket["title"], "Workflow Test");
    assert_eq!(final_ticket["description"], "Testing complete workflow");
    assert_eq!(final_ticket["status"], "Done");
}