fn main() {
    println!("Hello, world!");
    let mut age = 1;
    println!("age: {}", age);
    age = 2;
    println!("age: {}", age);
    let price = 9.0;
    println!("price: {}", price);
    let fee = 5;
    println!("fee: {}", fee);
    let total = price  + fee as f64;
    println!("total: {}", total);
    let message = String::from("Hello, world!");
    println!("message: {}", message);
    let m2 = "Hello, world!".to_string();
    println!("m2: {}", m2);
    let m3 = "hello";
    println!("m3: {}", m3);
    let m4 = format!("{} {} {}", m3, m2, total);
    println!("m4: {}", m4);
}
