use std::io;

pub struct Guess {
    age_in_day: u32,
}

impl Guess {
    pub fn default() -> Self {
        let mut days = if let age_in_year = Guess::read_age() {
            age_in_year*365
        } else {
            0
        };
        Self {
            age_in_day: days,
        }
    }

    pub fn run(&mut self) {
        println!("You are roughly {} days old.",self.age_in_day);
    }

    pub fn read_age() -> u32 {
        loop {
            println!("Please input your aget in years!");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Could not read age！ Make sure an integer is used.");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            return guess;
        }
    }

}