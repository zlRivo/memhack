use std::{thread, time::Duration};

fn main() {
    let mut i = 0;
    loop {
        println!("{i}");
        i += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
