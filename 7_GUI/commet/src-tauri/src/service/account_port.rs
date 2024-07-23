use crate::domain::accounts::account::Account;

pub trait AccountPort{
   fn new() -> Self;

   fn get_all_account(&self) -> Vec<Account> ;

   fn init_accounts(&self,accounts: Vec<Account>);
}