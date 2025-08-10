use rocket::{get, post, put};

#[get("/ticket/<id>")]
pub fn index(id: &str) -> &'static str {
    "Hello, world!"
}

#[post("/ticket")]
pub fn save() -> &'static str {
    "Hello, world!"
}

#[put("/ticket/<id>")]
pub fn change(id: &str) -> &'static str {
    "Hello, world!"
}
