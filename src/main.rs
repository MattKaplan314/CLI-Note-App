
use std::env;
use std::convert::TryFrom;
mod note;
mod request;
mod parse;
mod app;
use parse::req_parse;
use request::Request;
use app::App;

fn main() {
    let cycle = App::run();    
}





