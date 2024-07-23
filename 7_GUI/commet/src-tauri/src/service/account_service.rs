use crate::domain::accounts::account::Account;
use crate::infra::accounts::db_account_adapter::DBAccountAdapter;
use crate::service::account_port::AccountPort;

//https://tauri.app/v1/guides/features/command/
#[tauri::command]
pub fn init_accounts() {
    let account_port: DBAccountAdapter = AccountPort::new();

    let accounts = vec![Account {
        number: "14345454".parse().unwrap(),
        amount: 51548.654,
        sens: "D".parse().unwrap(),
    }];
    account_port.init_accounts(accounts)
}

#[tauri::command]
pub fn get_all_accounts() -> Vec<Account> {
    let account_port: DBAccountAdapter = AccountPort::new();
    account_port.get_all_account()
}
