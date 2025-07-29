fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    let mut i = low;

    while i <= high{
        *total += i;
        i += 1;
    }
}

fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut result = s.clone();
    result.push_str("World!");
    result
}

fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut result = s1.clone();
    result.push_str(s2);
    result
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"


    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

    let mut total: i32 = 0;
    sum(&mut total, 0, 100);
    println!("Sum 0 to 100: {}", total);
}
