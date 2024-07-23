use std::{fs, io};
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use rand::{Rng, RngCore};
use serde::{Deserialize, Deserializer, Serialize};
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

fn gen_account() -> Account {
    let mut rand_th = rand::thread_rng();
    Account {
        amount: BigDecimal::from_str(format!("{}.{}", rand_th.next_u32(), rand_th.next_u32()).as_str()).unwrap().round(2).to_string(),
        number: format!("{} {} {}", rand_th.gen_range(1000..9999).to_string(), rand_th.gen_range(1000..9999).to_string(), rand_th.gen_range(10000000000i64..99999999999i64).to_string()),
        sens: match rand_th.gen_range(0..1) {
            0 => D,
            1 => C,
            _ => { D }
        },
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

    return choice.trim().parse::<usize>().unwrap();
}