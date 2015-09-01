extern crate iron;
extern crate rustc_serialize;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;

#[derive(RustcDecodable, RustcEncodable)]
struct Job {
    id: String,
    msg: String
}

fn status(request: &mut Request) -> IronResult<Response> {
    let ref id = request.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
    let job = Job { id: id.to_string(), msg : "test".to_string() };
    let payload = json::encode(&job).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn set_job(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    request.body.read_to_string(&mut payload).unwrap();

    let request: Job = json::decode(&payload).unwrap();
    let job = Job { id: request.id, msg: request.msg };
    let payload = json::encode(&job).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn main() {
    let mut router = Router::new();

    router.get("/status/:id", status);
    router.post("/job", set_job);

    Iron::new(router).http("localhost:3000").unwrap();
}