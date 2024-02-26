
pub mod date {
    use chrono::{DateTime, FixedOffset, NaiveDateTime};

   pub fn make_timestamp_from_string(str: &str) -> Result<NaiveDateTime, &'static str> {

    // let result = NaiveDateTime::parse_from_str(str, "%Y %b %d %H:%M:%S%.3f %z")
    // .expect("parse error")
    // .un;
    
    // // Ok(NaiveDateTime::parse_from_str(str, "%Y %b %d %H:%M:%S%.3f %z").unwrap()
    // // .expect("bad date format"))

    // Ok(result)

    match NaiveDateTime::parse_from_str(str, "%Y %b %d %H:%M:%S%.3f %z") {
        Ok(parsed_date) => {
            Ok(parsed_date)
        },
        Err(e) => {
            Err("Parsing error")
        }
        
    }
   }
}