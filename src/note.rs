use chrono::{DateTime, Utc};
struct Note<'a> {
    path: &'a str,
    note: &'a  str,
    time: DateTime<Utc>,
}

impl<'a> Note<'a> {
    
}

