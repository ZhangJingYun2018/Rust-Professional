pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut amount = amount;
    let mut count = 0;
    while amount >=1 {
        let a = match amount {
            1..2 => 1,
            2..5 => 2,
            5..10 => 5,
            10..20 => 10,
            20..50 => 20,
            50..100 => 50,
            _ => 100,     
        };
        amount -= a;
        count += 1;
    }
    count
}
