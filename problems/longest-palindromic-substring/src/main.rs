fn longest_palindromic_string(s: String) -> String {
    s.chars().rev().collect()
}

fn main() {
    let st = String::from("cbbd");
    let result = longest_palindromic_string(st);

    println!("{result}");
    // assert_eq!(result, String::from("bb"));
}
