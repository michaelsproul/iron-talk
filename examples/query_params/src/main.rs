extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use urlencoded::UrlEncodedQuery;

fn handler(req: &mut Request) -> IronResult<Response> {
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(params) => {
            for (key, values) in params {
                println!("> {:?}: {:?}", key, values);
            }
            Ok(Response::with(format!("{:?}", params)))
        },
        Err(_) => Ok(Response::with("Error or no query given"))
    }
}

fn main() {
    Iron::new(handler).http("0.0.0.0:3000").unwrap();
}
