use std::time::Duration;

fn main() {
   loop {
       println!("Hello, world!");
       std::thread::sleep(Duration::from_secs(1));
   } 
}
