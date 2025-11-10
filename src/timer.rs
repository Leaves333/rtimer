use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    start: u64,
    end: u64,
}

impl Timer {
    pub fn duration(&self) -> u64 {
        self.end - self.start
    }
}

pub fn serialize_to_file(timer: Timer, filename: &str) {
    let mut file = match File::create(filename) {
        Ok(x) => x,
        Err(e) => {
            println!("failed to create file!");
            panic!("serialize_to_file: {e}");
        }
    };

    let str = serde_json::to_string(&timer).unwrap();
    file.write_all(str.as_bytes()).unwrap();
}

pub fn timer_from_file(filename: &str) -> Timer {
    todo!()
}
