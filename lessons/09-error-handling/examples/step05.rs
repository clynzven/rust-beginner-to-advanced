use thiserror::Error;

#[derive(Debug, Error)]
enum BankError {
    #[error("need {needed}, but only {available} available")]
    Insufficient { needed: u64, available: u64 },
    #[error("account is locked")]
    AccountLocked,
    #[error("could not parse amount")]
    BadAmount(#[from] std::num::ParseIntError),
}

fn withdraw(balance: u64, input: &str) -> Result<u64, BankError> {
    let amount: u64 = input.trim().parse()?;   // ParseIntError -> BankError
    if amount > balance {
        return Err(BankError::Insufficient { needed: amount, available: balance });
    }
    Ok(balance - amount)
}

fn main() {
    for input in ["30", "500", "ten"] {
        match withdraw(100, input) {
            Ok(left) => println!("{input}: ok, {left} left"),
            Err(e) => println!("{input}: {e}"),
        }
    }
    let _ = BankError::AccountLocked;
}
