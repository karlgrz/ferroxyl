extern crate iron;
extern crate rustc_serialize;
extern crate router;
extern crate persistent;

use iron::prelude::*;
use iron::typemap::Key;
use iron::status;

use persistent::Write;

use router::Router;
use rustc_serialize::json;
use std::io::Read;

#[derive(Copy, Clone)]
pub struct JobStore;

impl Key for JobStore { type Value = usize; }

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
struct Job {
    id: String,
    msg: String,
    count: usize
}

fn status(request: &mut Request) -> IronResult<Response> {
    let mutex = request.get::<Write<JobStore>>().unwrap();
    let mut count = mutex.lock().unwrap();

    *count += 1;

    let ref id = request.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
    let job = Job { id: id.to_string(), msg : "test".to_string(), count: *count };
    let payload = json::encode(&job).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn set_job(request: &mut Request) -> IronResult<Response> {
    let mutex = request.get::<Write<JobStore>>().unwrap();
    let mut count = mutex.lock().unwrap();

    let mut payload = String::new();

    request.body.read_to_string(&mut payload).unwrap();

    let request: Job = json::decode(&payload).unwrap();

    *count += request.count;

    let job = Job { id: request.id, msg: request.msg, count: *count };
    let payload = json::encode(&job).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn main() {

    let mut router = Router::new();

    router.get("/status/:id", status);
    router.post("/job", set_job);

    let mut chain = Chain::new(router);
    chain.link(Write::<JobStore>::both(0));

    Iron::new(chain).http("localhost:3000").unwrap();
}