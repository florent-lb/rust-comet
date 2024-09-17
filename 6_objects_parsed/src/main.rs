use std::{fs, io};
use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
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

    let file_path_str = "accounts.json";
    let path = Path::new(file_path_str);

    println!("Accounts load from {}", path.file_name().unwrap().to_str().unwrap());
    let content = fs::read_to_string(path).unwrap_or("{}".to_string());

    let file_with_account: HashMap<String, Vec<Account>> = serde_json::from_str(content.as_str()).unwrap();

    let accounts = file_with_account.get("accounts").unwrap();


    println!("Choose an account : ");

    let mut index = 0;
    for account in accounts {
        index = index + 1;
        println!("{} : number {}", index, account.number);
    }
    let chosen_account: Option<&Account> = accounts.get(input_number() - 1);
    match chosen_account {
        Some(chosen_account) => println!("You have {}{} on this account ({})", display_sens(&chosen_account.sens), chosen_account.amount, chosen_account.number),
        None => println!("No account found !")
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Account {
    amount: String,
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

#[derive(Deserialize, Serialize, Debug)]
enum SensCode {
    D,
    C,
}

fn input_number() -> usize {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read !");

    choice.trim().parse::<usize>().unwrap()
}