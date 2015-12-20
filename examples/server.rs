#![feature(plugin)]
#![plugin(maud_macros)]
extern crate maud;
extern crate iron;
extern crate maud_iron as mde;
//extern crate params;
use std::collections::HashMap;
mod tpl;
use iron::prelude::*;
use iron::mime::Mime;
use iron::{Handler,status};
//use params::Params;
use mde::{Template, MaudEngine};

fn handle(req: &mut Request) -> IronResult<Response> {
//    println!("{:?}", req.get_ref::<Params>());
//    let pm = req.get_ref::<Params>().unwrap();
//    for (key, val) in pm.iter() {
//        println!("key:{}", key);
//        match *val {
//            params::Value::Null => println!("{}", "null"),
//            params::Value::Boolean(value) => println!("bool:{:?}", value),
//            params::Value::I64(value)  => println!("i64:{:?}", value),
//            params::Value::U64(value)  => println!("u64:{:?}", value),
//            params::Value::F64(value)  => println!("f64:{:?}", value),
//            params::Value::String(ref value) => println!("String:{:?}", value),
//            params::Value::File(ref value) => println!("File:{:?}", value),
//            params::Value::Array(ref value) => println!("Array:{:?}", value),
//            params::Value::Map(ref value) => println!("Map:{:?}", value),
//        }
//    }
    let content_type = "text/html".parse::<Mime>().unwrap();
    let mut map = HashMap::new();
    map.insert("name", "Maud Iron plugin");
    map.insert("greating", "Great Maud Iron plugin!");
    let mut resp = Response::new();
 		resp.set_mut(content_type)
	 		.set_mut(Template::new(tpl::gethtml(&map)))
	 		.set_mut(status::Ok);
	Ok(resp)
}

fn form(req: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    let mut map = HashMap::new();
    map.insert("name", "Maud Iron plugin");
    map.insert("greating", "Great Maud Iron plugin!");
    let mut resp = Response::new();
 		resp.set_mut(content_type)
	 		.set_mut(Template::new(tpl::getform(&map)))
	 		.set_mut(status::Ok);
	Ok(resp)
}

struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path.join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}

fn main() {
    let mut router = Router::new();

    router.add_route("".to_string(), handle);

    router.add_route("form".to_string(), form);

    router.add_route("hello/again".to_string(), |_: &mut Request| {
       Ok(Response::with("Hello again !"))
    });

    router.add_route("error".to_string(), |_: &mut Request| {
       Ok(Response::with(status::BadRequest))
    });

    let mut chain = Chain::new(router);
		let mde = MaudEngine::new();
     chain.link_after(mde);

    Iron::new(chain).http("wram:8080").unwrap();
}
