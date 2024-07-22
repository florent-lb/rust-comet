use std::io;
use std::io::Read;

fn main() {
    println!("Display the banner of the day !");

    println!("
███████╗██╗  ██╗████████╗██╗ █████╗
██╔════╝╚██╗██╔╝╚══██╔══╝██║██╔══██╗
█████╗   ╚███╔╝    ██║   ██║███████║
██╔══╝   ██╔██╗    ██║   ██║██╔══██║
███████╗██╔╝ ██╗   ██║   ██║██║  ██║
╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝╚═╝  ╚═╝

███████╗ ██████╗ ██████╗     ██████╗ ██╗   ██╗███████╗████████╗
██╔════╝██╔═══██╗██╔══██╗    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝
█████╗  ██║   ██║██████╔╝    ██████╔╝██║   ██║███████╗   ██║
██╔══╝  ██║   ██║██╔══██╗    ██╔══██╗██║   ██║╚════██║   ██║
██║     ╚██████╔╝██║  ██║    ██║  ██║╚██████╔╝███████║   ██║
╚═╝      ╚═════╝ ╚═╝  ╚═╝    ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝ ");

    println!("What's say to the death ?");
    println!("1 - Not Today !");
    println!("2 - Not Tomorrow !");
    println!("3 - Not Yesterday !");


    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Failed to read! ");

        let num_choice = choice.trim().parse::<i32>().unwrap();
        if num_choice == 1 {
            println!("you GOT this one! ");
            break;
        } else {
            println!("Not Today! ")
        }
    }
}
