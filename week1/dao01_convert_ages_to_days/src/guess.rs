use std::io;

#[derive(Default)]
pub struct Guess {
    age_in_days: u32,
}

impl Guess {
    pub fn run(&mut self) {
        self.read_age();
        println!("You are roughly {} days old.",self.age_in_days);
    }

    pub fn read_age(&mut self) -> Result<(),io::Error> {
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
            self.age_in_days = guess * 365;
            return Ok(());
        }
    }

}