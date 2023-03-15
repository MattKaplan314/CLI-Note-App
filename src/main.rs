
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






pub fn combine_vec_items(vec: Vec<String>) -> String {
    let mut ret: String = "".to_string();

    for i in vec {
        ret.push_str(&i);
        ret.push_str(" ");
    }

    ret.to_string()
}



