use std::io;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use rand::{Rng, RngCore};

use crate::SensCode::{C, D};

fn main() {
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


    println!("Choose an account : ");
    let nb_account = rand::thread_rng().gen_range(2..5);
    let mut index = 0;
    let mut accounts: Vec<Account> = Vec::new();
    while index < nb_account {
        index = index + 1;
        accounts.push(gen_account());
        println!("{index} : number {}", accounts.last().unwrap().number);
    }

    let chosen_account: Option<&Account> = accounts.get(input_number()-1);
    match chosen_account {
        Some(chosen_account) => println!("You have {}{} on this account ({})", display_sens(&chosen_account.sens), chosen_account.amount, chosen_account.number),
        None => println!("No account found !")
    }
}

fn gen_account() -> Account {
    let mut rand_th = rand::thread_rng();
    Account {
        amount: BigDecimal::from_str(format!("{}.{}",rand_th.next_u32(),rand_th.next_u32()).as_str()).unwrap().round(2),
        number: format!("{} {} {}", rand_th.gen_range(1000..9999).to_string(), rand_th.gen_range(1000..9999).to_string(), rand_th.gen_range(10000000000i64..99999999999i64).to_string()),
        sens: match rand_th.gen_range(0..1) {
            0 => D,
            1 => C,
            _ => { D }
        },
    }
}

struct Account {
    amount: BigDecimal,
    number: String,
    sens: SensCode,
}

fn display_sens(sens: &SensCode) -> &str
{
    match sens {
        C => "+",
        D => "-"
    }
}

enum SensCode {
    D,
    C,
}

fn input_number() -> usize {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read !");

    return choice.trim().parse::<usize>().unwrap()
}