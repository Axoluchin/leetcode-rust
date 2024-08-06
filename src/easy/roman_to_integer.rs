use std::collections::HashMap;

pub fn roman_to_integer(s: String) -> i32 {
    let mut roman_values = HashMap::new();
    roman_values.insert("I".to_string(), 1);
    roman_values.insert("V".to_string(), 5);
    roman_values.insert("X".to_string(), 10);
    roman_values.insert("L".to_string(), 50);
    roman_values.insert("C".to_string(), 100);
    roman_values.insert("D".to_string(), 500);
    roman_values.insert("M".to_string(), 1000);

    let mut result = 0;
    let mut last_number = 0;

    for value in s.chars().rev() {
        let new_value = roman_values[&value.to_string()];

        if new_value >= last_number {
            result += new_value;
            last_number = new_value;
        } else {
            result -= new_value;
        }
    }

    return result;
}
