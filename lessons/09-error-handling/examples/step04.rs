use std::fmt;

#[derive(Debug)]
enum BankError {
    Insufficient { needed: u64, available: u64 },
    AccountLocked,
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BankError::Insufficient { needed, available } =>
                write!(f, "need {needed}, but only {available} available"),
            BankError::AccountLocked => write!(f, "account is locked"),
        }
    }
}

impl std::error::Error for BankError {}

fn withdraw(balance: u64, amount: u64, locked: bool) -> Result<u64, BankError> {
    if locked {
        return Err(BankError::AccountLocked);
    }
    if amount > balance {
        return Err(BankError::Insufficient { needed: amount, available: balance });
    }
    Ok(balance - amount)
}

fn main() {
    println!("{:?}", withdraw(100, 30, false));
    for (amount, locked) in [(500, false), (10, true)] {
        if let Err(e) = withdraw(100, amount, locked) {
            println!("error: {e}");
        }
    }
}
