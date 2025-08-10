use rocket::{catch, http::Status, response::{content, status}, Request};

#[catch(default)]
pub fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

#[catch(404)]
pub fn general_not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"<p>404 Not found... What are you looking for?</p>"#)
}
