pub fn new_count_distinct(input_str: &str) -> usize {
    let mut set = std::collections::HashSet::new();
    for c in input_str.split(",") {
        set.insert(c);
    }
    set.len()
}
