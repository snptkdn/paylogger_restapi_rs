use rocket::http::{Header, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response;
use rocket::serde::json::Json;
use rocket::State;

pub struct PfResponse;
impl<'r> response::Responder<'r, 'static> for PfResponse {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'static> {
        response::Response::build()
            .header(Header::new("Access-Control-Allow-Origin", "*"))
            .header(Header::new(
                "Access-Control-Allow-Methods",
                "POST,GET,PATCH,OPTIONS,TRACE",
            ))
            .header(Header::new("Access-Control-Allow-Headers", "x-api-key"))
            .header(Header::new("Access-Control-Allow-Credentials", "true"))
            .ok()
    }
}
