use std::io;

fn main() {
    let mut stringinput = String::new();
    println!("Enter in something that i will repeat back to you:");
    io::stdin().read_line(&mut stringinput);
    println!("{}",stringinput)
}
