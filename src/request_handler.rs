use hyper::{Client, Request, Response, StatusCode};
use hyper::body::Body;
use log::{debug, info, error};
use std::convert::Infallible;
use regex::Regex;
use crate::request_filter::filter_request;
use crate::settings::RequestPlugins;


pub async fn handle_request(mut req: Request<Body>) -> Result<Response<Body>, Infallible>  {
    let (svr_addr, prefix_to_remove, req_plugins) = match_uri(req.uri().to_string());
    
    //--- BEGIN Extract Request headers ---//
    let mut req_headers_str = String::new();
    for (key, value) in req.headers() {
        match value.to_str() {
            Ok(value_str) => {
                req_headers_str.push_str(&format!("{}: {}\n", key, value_str));
            }
            Err(_) => {
                req_headers_str.push_str(&format!("{}: <invalid UTF-8>\n", key));
            }
        }
    }
    //--- END Extract Request headers ---//

    //--- BEGIN Extract Request body ---//
    let whole_body = hyper::body::to_bytes(req.body_mut()).await.unwrap();

    // Convert the bytes into a String
    let body_str = String::from_utf8(whole_body.to_vec()).unwrap();
    
    //--- END Extract Request body ---//

    //Log Request body if turned on
    if super::SETTINGS.clone().get_logging().get_log_request_header_and_body().to_lowercase().eq("true"){
        debug!("Response:{}Headers:{}{}{}Body:{}{}","\n","\n",req_headers_str,"\n","\n",body_str);
    }

    let req_copy = filter_request(req, req_plugins);

    proxy_request(req_copy, &svr_addr, prefix_to_remove).await
}

fn match_uri(uri:String) -> (String, String, Vec<RequestPlugins>){
    //let s = Settings::new().expect("Unalve to instantiate settings");
    let s = super::SETTINGS.clone();
    let sm = s.get_service_mapping();
    let mut backend_address: String = "".to_string();
    let mut prefix_uri_to_remove = "".to_string();
    let mut request_plugins: Vec<RequestPlugins> = Vec::new();

    #[allow(unused_variables)]
    let u = uri.clone();

    for svc in sm{
        
        let re: Regex= match Regex::new(&svc.clone().get_url_matching_expression()){
            Ok(regex) => regex,
            Err(err) => {
                eprintln!("Error compiling regex: {}", err);
                return ("".to_string(),"".to_string(), Vec::new());
                
        }};
        let local_uri = uri.clone();
        if re.is_match(&local_uri) {
            backend_address=svc.clone().get_service_address();
            prefix_uri_to_remove = svc.clone().get_backend_prefix_removal();
            request_plugins = svc.clone().get_request_plugins();
            break;
        }
    }
    (backend_address,prefix_uri_to_remove, request_plugins)
}

fn remove_prefix_from_uri(uri: &str, prefix: &str) -> String {
    if uri.starts_with(prefix) {
        // Remove the prefix by slicing the string from the end of the prefix
        uri[prefix.len()..].to_string()
    } else {
        // Return the original URI if the prefix is not at the start
        uri.to_string()
    }
}

fn remove_extra_slashes(url: &str) -> String {
    // Split the URL into protocol and the rest
    if let Some((protocol, rest)) = url.split_once("://") {
        // Define a regular expression to match multiple consecutive slashes
        let re = Regex::new(r"/{2,}").unwrap();
        // Replace any occurrence of multiple slashes in the rest of the URL with a single slash
        let cleaned_rest = re.replace_all(rest, "/");
        // Reconstruct the URL
        format!("{}://{}", protocol, cleaned_rest)
    } else {
        // If the URL does not contain "://", just return it as is
        url.to_string()
    }
}

pub async fn proxy_request(req: Request<Body>, svc_addr:&str, prefix_to_remove:String) -> Result<Response<Body>, Infallible>  {
    let req_uri = req.uri().to_string();
    
    //println!("{}", req_uri.clone());
    info!("Request uri: {}", req_uri);
    
    let mut dest_full_address = svc_addr.to_owned();
    //remove prefix from uri
    let nuri = remove_prefix_from_uri(&req_uri,&prefix_to_remove);
    
    //Build detination uri
    dest_full_address.push_str(nuri.as_str());

    let dest_full_address_no_double_slashes = remove_extra_slashes(dest_full_address.as_str());
    info!("URI no formatting: {}", dest_full_address.as_str());
    info!("URI no formatting: {}", dest_full_address_no_double_slashes.as_str());
    
    // Parse the URI
    //let url: Uri = dest_full_address.parse::<Uri>().expect("Failed to parse the URI");
    info!("Forwarded to: {}", nuri.clone());
    // Create a client
    let client = Client::new();

    // Forward the request to the specified URL
    let (mut parts, body) = req.into_parts();
    parts.uri = dest_full_address_no_double_slashes.parse().expect("Unable to parse requested uri");
    let new_req = Request::from_parts(parts, body);

    // Send the request using the Hyper client
    let res = match client.request(new_req).await {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to send request to backend service: {:?}", e);
             Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found"))
                .expect("Failed to create 404 response")
            
        }
    };
    
    //let headers = res.headers();
    let mut headers_str = String::new();
    for (key, value) in res.headers() {
        match value.to_str() {
            Ok(value_str) => {
                headers_str.push_str(&format!("{}: {}\n", key, value_str));
            }
            Err(_) => {
                headers_str.push_str(&format!("{}: <invalid UTF-8>\n", key));
            }
        }
    }

    // Convert the response body into Full<Bytes>
    let full_body = hyper::body::to_bytes(res.into_body()).await.expect("Failed to read the full response body");
    
    let body_str = String::from_utf8(full_body.to_vec()).expect("Response body is not valid UTF-8");
    
    if super::SETTINGS.clone().get_logging().get_log_response_header_and_body().to_lowercase().eq("true"){
        debug!("Response:{}Headers:{}{}{}Body:{}{}","\n","\n",headers_str,"\n","\n",body_str);
    }

    Ok(Response::new(full_body.into()))
}