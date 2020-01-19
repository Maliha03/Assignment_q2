
mod lib;
use std::io;

fn main() {

lib::temp::room::ac();

    let temp : u8 = user_ip();
    println!("Room temperature = {} C\n", temp)

}

fn user_ip () -> u8 {

    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Failed to read line");
    let input : u8 = ip.trim().parse().expect("Please type a number!"); 
    input
}
