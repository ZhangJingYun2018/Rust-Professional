pub fn new_birthday_probability(n: u32) -> f64 {
    assert!(n > 1);
    let mut p = 1.0;
    for i in 1..n {
        p *= (365.0 - i as f64) / 365.0;
    }
    //保留四位小数
    ((1.0 - p) * 10000.0).round() / 10000.0
}
