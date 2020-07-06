extern crate rand;

use rand::Rng;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut robot = Self {
            name: String::new(),
        };
        robot.reset_name();
        return robot;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let mut new_name = String::new();
        new_name.push_str(
            &(0..2)
                .map(|_| rand::thread_rng().gen_range(65, 90) as u8)
                .map(|i| i as char)
                .collect::<String>(),
        );
        new_name.push_str(
            &(0..3)
                .map(|_| rand::thread_rng().gen_range(0, 9).to_string())
                .collect::<String>(),
        );

        self.name = new_name
    }
}
