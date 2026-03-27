mod logic;

use logic::{buffer::Buffer, parser::get_verses};
use std::thread;
use std::time::Duration;

fn main() {
    let mut buffer = Buffer::new();
    loop {
        let input = buffer.read_line();

        println!("{:?}", get_verses(&input));

        thread::sleep(Duration::from_secs(5));
    }
}
