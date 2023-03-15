// app.rs will handel the entire program 
// this will be the part that touches the file system 
//
// BASIC OVERVIEW
// --------------
//
// check action
//
// for each action have correct "program cycle"
//
use crate::request::Request;
use crate::req_parse::{ user_action, user_action_error };
use std::io::Write;
use std::{env, fs};
use std::convert::TryFrom;
use std::fs::{File, Metadata};
pub struct App<'a> {
    current_request: &'a Request<'a>,
}


impl<'a> App<'a> {
   pub fn run() {
        //find note directory
        let note_dir_head = "~/notes/";
        // if not there aka first time running program than it will make the new dir
        if path_exists(note_dir_head) == false {
               fs::create_dir(note_dir_head); 
        }


        let args: Vec<String> = env::args().collect();
        let inter_string: String = crate::combine_vec_items(args);
        let byte_array = inter_string.as_bytes();
        let request_result = Request::try_from(byte_array);
        let current_request = request_result.unwrap();

        
        //parse Request
        if current_request.command == "new" {                // make new file
                let mut file = File::create(current_request.path).expect("this file dont exist buddy!");
                // write to file
                file.write(current_request.note.as_bytes()); 
        }

        if current_request.command == "view" {
            let contents = fs::read_to_string(current_request.path).expect("Should have been able to read the file");
            println!("{}", contents);
        }

        if current_request.command == "delete" {  
            println!("{}", "go delet it your self dickhead");

   }
        if current_request.command == "help" {
            println!("{}", "to use new command: >>> app.exe new path-include file name with ext- note");
            println!("{}", "to use view command: >>> app.exe new path-include file name with ext- note");
            println!("{}", "to use delet command: >>> app.exe new path-include file name with ext- note");
        }
}
}
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
