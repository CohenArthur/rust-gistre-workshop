mod bank;
mod money;

use bank::Bank;
use money::{Dollar, Euro, Ouguiya};

fn main() {
    let mut bank_of_america = Bank::new();
    bank_of_america.add_account("Trisha Paytas");
    bank_of_america.add_money("Trisha Paytas", Dollar::new(16000.0));
    bank_of_america.add_money("Joe Biden", Dollar::new(99999999999999.0));

    let mut bnp = Bank::new();
    bnp.add_account("Jojo");
    bnp.add_money("Jojo", Euro::new(1600));

    let mut maybank = Bank::new();
    maybank.add_account("Jimmy Choo");
    maybank.add_money("Jimmy Choo", Ouguiya::new(u64::MAX));

    println!("Bank of America: {}", bank_of_america);
    println!("BNP Paribas: {}", bnp);
    println!("Maybank: {}", maybank);
}
