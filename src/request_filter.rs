use hyper::Request;
use hyper::body::Body;
use log::{debug};
use libloading::{Library, Symbol};
use std::sync::Arc;

use crate::settings::RequestPlugins;
type HandleRequestFn = fn(Request<Body>) -> Request<Body>;

pub fn filter_request(mut req: Request<Body>, req_plugins: Vec<RequestPlugins>) -> Request<Body> {

    
    debug!("BEGIN - Applying request plugins...");
    for p in req_plugins {
        debug!("Plugin name: {}", p.clone().get_name());

        // Load the shared library
        let lib_path = p.get_name(); // Adjust the path as necessary
        let lib = unsafe { Library::new(lib_path).unwrap() };

        // Get the handle_request function from the library
        let handle_request: Symbol<HandleRequestFn> = unsafe { lib.get(b"handle_request").unwrap() };
        let handle_request = Arc::new(handle_request);
        req = handle_request(req);


    }
    debug!("END - Applying request plugins.");

    req
}

/*
fn extract_header(req: &Request<Body>, header_name: &str) -> Option<String> {
    req.headers().get(header_name).and_then(|value| {
        value.to_str().ok().map(|s| s.to_string())
    })
}
 */