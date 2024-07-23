use crate::domain::accounts::account::Account;
use crate::infra::accounts::db_account_adapter::DBAccountAdapter;
use crate::service::account_port::AccountPort;

#[tauri::command]
pub fn init_accounts(){

    let account_port : DBAccountAdapter = AccountPort::new();

    let accounts = vec![Account{
        number: "14345454".parse().unwrap(),
        amount : 51548.654,
        sens : "D".parse().unwrap()
    }];
    println!("Intialize data with accounts");
   account_port.init_accounts(accounts)

}
