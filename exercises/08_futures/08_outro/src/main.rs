use rocket::{
    http::Status,
    response::{content, status},
    Build, Request, Rocket,
};

#[macro_use]
extern crate rocket;

#[get("/ticket/<id>")]
fn index(id: &str) -> &'static str {
    "Hello, world!"
}

#[post("/ticket")]
fn save() -> &'static str {
    "Hello, world!"
}

#[put("/ticket/<id>")]
fn change(id: &str) -> &'static str {
    "Hello, world!"
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

#[catch(404)]
fn general_not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"<p>404 Not found... What are you looking for?</p>"#)
}

fn rocket() -> Rocket<Build> {
    rocket::build()
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
