use std::io;


fn main() {
    let mut str1: String = String::new();
    let mut str2: String = String::new();

    println!("Enter the 1st string");
    io::stdin().read_line(&mut str1).expect("Failed to read 1st string");

    println!("Enter the second string");
    io::stdin().read_line(&mut str2).expect("Failed to read the 2nd string");

    if is_permutation(&str1.to_uppercase(), &str2.to_uppercase()){
        println!("The given strings are interchanagble")
    } else {
        println!("The given strings are not interchangable")
    }
}

fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut sorted_s1 = s1.chars().collect::<Vec<char>>();

    let mut sorted_s2 = s2.chars().collect::<Vec<char>>();

    sorted_s1.sort();
    sorted_s2.sort();

    return sorted_s1 == sorted_s2;
}
