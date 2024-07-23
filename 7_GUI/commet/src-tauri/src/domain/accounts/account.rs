use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Account{
    pub number: String,
    pub sens : String,
    pub amount : f64
}