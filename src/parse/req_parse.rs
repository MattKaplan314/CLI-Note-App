//this will handle all requests to make sense of what the user wants
// will be a pretty big file but i think i can figure
//out how to make it small

// ############# idea stage ###################

// it will take a request and turn each part that makes sense to turn into
// an enum, into an enum, after that im not sure yet but after i set
// a new "state " as to what the user will be doing
// i will either open a new file or find an old one to view
// i dont ahve to worry about editing exsitig files because the point of this app
// is to quickly get thoughts out as your working, and to be able
// to go back to look at them later to use for work or whatever your workin gon where
// quick notes can make sense to help you stay organized but stay creative 

// ################ more thinking ######################
// things that that are "safe"/ things that i have taken from the user and turned into a type that
// the system can then work with 
// -----------------------------
//  - request struct that spilts user entered arguments
//  -------------------------------------------------
// what needs to happen in the app
// -------------------------------
//  check if the user either said new or view or delete
//  // if new then check if the user added a path, then if path exsits and if neither of thoes
//  things are true then it will make a new file that contains the new note


// ############# code #####################

use std::str::FromStr;

use crate::request::Request;

pub enum user_action {
    New,
    View,
    Delete,
    Help,
}
// this will allow the system to turn user requests into individual parts that will
// control what happens next
impl FromStr for user_action {
    type Err = user_action_error;

    fn from_str(to_parse: &str) -> Result<Self, Self::Err> {
        match to_parse {
            "new" => Ok(Self::New),
            "view" => Ok(Self::View),
            "delete" => Ok(Self::Delete),
            "help" => Ok(Self::Help),
            _ => Err(user_action_error),
        }
    }

    //per command variant i need to then

}

pub struct user_action_error;

