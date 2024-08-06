// https://leetcode.com/problems/palindrome-number

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {return false};
    
    let num_as_string = x.to_string();

    return num_as_string == num_as_string.chars().rev().collect::<String>();
}