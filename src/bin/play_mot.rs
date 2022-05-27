fn main() {
    let x = -5;
    println!("+5: {:b}", 5 as i32);
    println!("-5: {:b}", x);
    let x = x.min(0);
    println!("{:b}", x as u32);
    println!("{}", u32::MAX);
}
