#![feature(proc_macro_hygiene, decl_macro)]
use rocket::response::content;
use rocket::http::RawStr;
use std::env;
use std::process;

#[macro_use] extern crate rocket;

static TOKEN_ENV: &str = "TOKEN";


fn to_success_json(msg: &String) -> content::Json<String> {
    let beginning: String = " { \"success\": \"".into();
    let end: String = "\" }".into();
    return content::Json(format!("{}{}{}", beginning, msg, end));
}

fn to_error_json(msg: &String) -> content::Json<String> {
    let beginning: String = " { \"error\": \"".into();
    let end: String = "\" }".into();
    return content::Json(format!("{}{}{}", beginning, msg, end));
}

fn decode_url(raw_url: &RawStr) -> Result<String, String> {
    return match raw_url.url_decode() {
        Ok(val) => Ok(val),
        Err(e) => Err(format!(
            "Unable decode URL, utf-8 error. Double check your strings and try again. {}",
            e.to_string()
        ))
    };
}

fn test_token(token: &String) -> bool {
    return match env::var(&TOKEN_ENV) {
        Ok(val) => &val == token,
        Err(_) => false
    }
}

#[get("/url/<raw_url>/<token>")]
fn index(raw_url: &RawStr, token: String) -> content::Json<String> {
    if !test_token(&token) {
        return to_error_json(&"Invalid token".into())
    }
    return match decode_url(raw_url) {
        Ok(decoded_url) => {
            match send_request(&decoded_url) {
                Ok(status_code) => to_success_json(&status_code),
                Err(http_request_error) => to_error_json(&http_request_error),
            }
        }
        Err(error_decoding_url) => to_error_json(&error_decoding_url)
    };
}


fn send_request(url: &String) -> Result<String, String> {
    return match reqwest::blocking::get(url) {
        Ok(r) => Ok(r.status().as_u16().to_string()),
        Err(e) => Err(format!(
            "Http request error: {}", e.to_string()
        ))
    }
}


fn main() {
    if env::var(&TOKEN_ENV).is_err() {
        println!("{}", format!("{} not set. Exiting.", &TOKEN_ENV));
        process::exit(1);
    }
    rocket::ignite().mount("/", routes![index]).launch();
}
