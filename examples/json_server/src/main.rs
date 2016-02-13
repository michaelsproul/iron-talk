extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable)]
struct Process {
    pub name: String,
    pub id: i32,
    pub children: Vec<i32>
}

fn main() {
    let handler = |_req: &mut Request| {
        let process = Process {
            name: "Hello world!".to_string(),
            id: 1,
            children: vec![2, 4, 8, 16]
        };
        let body = json::encode(&process).unwrap();
        Ok(Response::with(body))
    };
    Iron::new(handler).http("0.0.0.0:3000").unwrap();
}
