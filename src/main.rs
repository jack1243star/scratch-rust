extern crate chrono;

fn main() {
    // Know the week number!
    {
        use chrono::{DateTime, Datelike, Local};
        let local: DateTime<Local> = Local::now();
        println!("It's week {} now!", local.isoweekdate().1);
    }
}
