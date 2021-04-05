mod bank;
use bank::Bank;

fn main() {
    let mut bank_of_america = Bank::new();
    bank_of_america.add_account("Trisha Paytas");
    bank_of_america.add_money("Trisha Paytas", 150000);

    bank_of_america.add_money("Joe Biden", 99999999999999);

    println!("Bank of America: {}", bank_of_america);
}
