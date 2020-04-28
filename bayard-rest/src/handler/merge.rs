use iron::headers::ContentType;
use iron::prelude::*;
use iron::{status, IronError, IronResult, Request, Response};
use persistent::Write;

use crate::handler::Client;

pub fn merge(req: &mut Request) -> IronResult<Response> {
    let client = req.get::<Write<Client>>().unwrap();
    let mut index_client = client.lock().unwrap();
    match index_client.merge() {
        Ok(()) => Ok(Response::with((ContentType::plaintext().0, status::Ok, ""))),
        Err(e) => Err(IronError::new(
            e,
            (status::InternalServerError, "failed to merge index"),
        )),
    }
}