extern crate chrono;
extern crate rand;

#[derive(Debug)]
enum Zundoko {
    Zun,
    Doko,
}

impl std::fmt::Display for Zundoko {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Zundoko::Zun => write!(f, "ズン"),
            Zundoko::Doko => write!(f, "ドコ"),
        }
    }
}

impl rand::Rand for Zundoko {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        if rng.gen() {
            return Zundoko::Zun;
        } else {
            return Zundoko::Doko;
        }
    }
}

fn main() {
    // Know the week number!
    {
        use chrono::{DateTime, Datelike, Local};
        let local: DateTime<Local> = Local::now();
        println!("It's week {} now!", local.isoweekdate().1);
    }

    // Play Zundoko
    {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut zuncount = 0_u8;
        let mut time = 1_u32;
        loop {
            let zundoko: Zundoko = rng.gen();
            print!("{} ", zundoko);
            match zundoko {
                Zundoko::Zun =>
                    zuncount =
                        if zuncount == 4 {
                            zuncount
                        } else {
                            zuncount+1
                        },
                Zundoko::Doko =>
                    if zuncount == 4 {
                        break;
                    } else {
                        zuncount = 0;
                    }
            }
            time = time+1;
        }
        println!("キ・ヨ・シ！ ({}回)", time);
    }
}
