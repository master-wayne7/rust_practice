struct MasterCard {
    number: u8,
    verification: u8,
}
struct Visa {
    number: u16,
}
struct WesternUnion {
    name: String,
    verification: u8,
}
struct BitCredit {
    btcnumber: u32,
}

impl CreditCharge for WesternUnion {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 3 == (self.verification % 4) as u32
    }
}
impl CreditCharge for BitCredit {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 2 == self.btcnumber % 2
    }
}

trait CreditCharge {
    fn charge_with_id(&self, id: u32) -> bool;
}

fn transact<Q: CreditCharge>(card: Q) {
    let id = 4096;
    if card.charge_with_id(id) {
        println!("success");
    } else {
        println!("Invalid code");
    };
}

fn main() {
    let card = BitCredit { btcnumber: 1024 };
    // let code = 4096;

    // if card.charge_with_id(code) {
    //     println!("success");
    // } else {
    //     println!("failure");
    // }\
    transact(card);
}
