use rusqlite::Connection;
use crate::domain::accounts::account::Account;
use crate::service::account_port::AccountPort;

pub struct DBAccountAdapter {
    connection: Connection,
}

impl DBAccountAdapter {}
fn get_connection() -> Connection {
    Connection::open_in_memory().unwrap()
}
impl AccountPort for DBAccountAdapter {
    fn new() -> DBAccountAdapter {
        DBAccountAdapter {
            connection: get_connection(),
        }
    }

    fn get_all_account(&self) -> Vec<Account> {
        let mut result_set = self.connection.prepare("SELECT number,amount,sens FROM accounts");

        let accounts = result_set.unwrap().query_row([], |row| {
            Ok(
                Account {
                    number: row.get(0)?,
                    amount: row.get(1)?,
                    sens: row.get(2)?,
                }
            )
        });

        let mut all_accounts: Vec<Account> = vec![];
        for account in accounts{
            all_accounts.push(account);
        }
        all_accounts
    }

    fn init_accounts(&self, accounts: Vec<Account>) {
        println!("Intialize DB with accounts");
        self.connection.execute("
                                CREATE TABLE accounts (
                                number  TEXT
                                amount  NUMERIC
                                sens    TEXT
                                )
                                ", ()).unwrap();


        for account in accounts {
            self.connection.execute("INSERT INTO accounts (number,amount,sens) VALUES (?1, ?2, ?3)",
                                    (&account.number, &account.amount, &account.sens)).unwrap();
        }
    }
}