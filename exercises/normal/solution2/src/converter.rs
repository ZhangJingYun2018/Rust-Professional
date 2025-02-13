pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // Convert the number string to a vector of digits
    //先拆解num_str字符串，得到原始进制
    let mut v = vec![];
    //去掉num_str的最后一位 ) 获取原始数字和进制
    for c in (&num_str[0..num_str.len() - 1]).split("(") {
        v.push(c);
    }
    let (num_str, base_str) = (v[0], v[1]);
    let base: u32 = base_str.parse().unwrap();
    //转换成10进制
    let mut num_dec = 0;
    for (i, c) in num_str.chars().enumerate() {
        let digit = c.to_digit(base).unwrap();
        num_dec += digit * base.pow(num_str.len() as u32 - 1 - i as u32);
    }
    //转换成目标进制
    if to_base == 10 {
        return num_dec.to_string();
    }
    decimal_to_base(num_dec, to_base)
}

fn decimal_to_base(mut num: u32, base: u32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let hex_chars = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let mut result = String::new();
    while num > 0 {
        let remainder = num % base;
        num /= base;
        result.push(hex_chars[remainder as usize]);
    }
    result.chars().rev().collect()
}
