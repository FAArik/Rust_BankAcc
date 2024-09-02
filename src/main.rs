
trait Account{
    fn deposit(&mut self,amount:f32);
    fn withdraw(&mut self,amount:f32);
    fn balance(&self) ->f32;
}

struct BankAccount{
    account_number:String,
    holder_name:String,
    balance:f32,
}
impl Account for BankAccount {
    fn deposit(&mut self,amount:f32) {
        self.balance += amount;
    }

    fn withdraw(&mut self,amount:f32) {
        self.balance -= amount;
    }

    fn balance(&self) ->f32{
        self.balance
    }
}
fn main(){

    let mut fatih_acc =BankAccount{
        balance:0.00,
        account_number:"951147".to_string(),
        holder_name:"Fatih".to_string()
    };
    let mut ali_acc =BankAccount{
        balance:0.00,
        account_number:"741159".to_string(),
        holder_name:"Ali".to_string()
    };
    fatih_acc.deposit(10.00);
    ali_acc.withdraw(10.00);
    println!("Acc1 balance : {}", fatih_acc.balance());
    println!("Acc2 balance : {}", ali_acc.balance());
}

