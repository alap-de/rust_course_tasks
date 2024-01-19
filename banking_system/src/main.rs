fn main() {
    let mut johns_bank_account = BankAccount {
        _account_number: 123,
        _holder_name: "John".to_string(),
        balance: 100.0,
    };
    let mut janes_bank_account = BankAccount {
        _account_number: 270,
        _holder_name: "Jane".to_string(),
        balance: 200.0,
    };

    let handle_deposit_result = handle_result("Deposit successful", "Deposit failed");
    let handle_withdraw_result = handle_result("Withdrawal successful", "Withdrawal failed");

    handle_deposit_result(johns_bank_account.deposit(100.0));
    handle_withdraw_result(janes_bank_account.withdraw(50.0));
    handle_deposit_result(johns_bank_account.deposit(-100.0));
    handle_withdraw_result(janes_bank_account.withdraw(50000.0));

    println!(
        "John's bank account balance: {}",
        johns_bank_account.balance()
    );
    println!(
        "Jane's bank account balance: {}",
        janes_bank_account.balance()
    );
}

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    _account_number: u32,
    _holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(format!("Invalid amount {}", amount));
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            return Err("Insufficient balance".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn handle_result<'a>(success_msg: &'a str, error_msg: &'a str) -> impl Fn(Result<(), String>) + 'a {
    move |result: Result<(), String>| match result {
        Ok(()) => println!("{}", success_msg),
        Err(err) => println!("{}: {}", error_msg, err),
    }
}
