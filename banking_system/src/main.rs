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

    johns_bank_account.deposit(100.0);
    janes_bank_account.withdraw(50.0);

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
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    _account_number: u32,
    _holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            panic!("Insufficient balance");
        }
        self.balance -= amount;
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
