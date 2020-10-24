use anat::NatNum;

fn main() {
    let num1 = NatNum::from(1);
    let num2 = NatNum::from(5);
    let num3 = num1.add_rec(&num2);
    
    println!("num1 = {}", num1.to_number());
    println!("num2 = {}", num2.to_number());
    println!("num3 = num1 + num2 = {}", num3.to_number());
    println!("rust representation of num3:\n{:?}", num3);
    println!("num3 as written empty sets:\n{}", num3.to_string());
}