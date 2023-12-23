use std::collections::HashMap;

pub fn create_number_map() -> HashMap<&'static str, usize> {
    let mut hash_map = HashMap::new();
    hash_map.insert("one", 1);
    hash_map.insert("two", 2);
    hash_map.insert("three", 3);
    hash_map.insert("four", 4);
    hash_map.insert("five", 5);
    hash_map.insert("six", 6);
    hash_map.insert("seven", 7);
    hash_map.insert("eight", 8);
    hash_map.insert("nine", 9);
    hash_map.insert("1", 1);
    hash_map.insert("2", 2);
    hash_map.insert("3", 3);
    hash_map.insert("4", 4);
    hash_map.insert("5", 5);
    hash_map.insert("6", 6);
    hash_map.insert("7", 7);
    hash_map.insert("8", 8);
    hash_map.insert("9", 9);
    hash_map
}
