struct BankAccount {
    balance: i32,
    verified: bool,
}

fn print_balance(account: &BankAccount) {
    println!("{:?}", account.balance);
}

fn print_verifeid(account: &BankAccount) {
    println!("{:?}", account.verified);
}

fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    match account.verified {
        true => Ok(true),
        false => Err(false),
    }
}

fn main() {
    let my_account = BankAccount {
        balance: 30,
        verified: false,
    };
    let verification_stattus = is_verified(&my_account).expect("Unable to unwarp resutl");

    print_balance(&my_account);
    print_verifeid(&my_account);

    println!("{:?}", verification_stattus);
}
