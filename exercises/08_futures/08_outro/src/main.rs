// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.


use outro_08::domain::store::TicketStore;
use outro_08::routes::infra::{default_catcher, general_not_found};
use outro_08::routes::ticket::{change, index, save};
use rocket::{Build, Rocket};


#[macro_use]
extern crate rocket;


fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(TicketStore::new())
        .mount("/api", routes![index, save, change])
        .register("/", catchers![general_not_found, default_catcher])
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}
