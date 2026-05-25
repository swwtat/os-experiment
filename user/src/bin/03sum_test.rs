#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[unsafe(no_mangle)] // 适配 Rust 2024 新版语法规范
fn main() -> i32 {
    println!("============== APP 3: Sum Test Start ==============");

    let mut sum = 0;
    let target = 100;

    println!("Calculating the sum from 1 to {}...", target);

    for i in 1..=target {
        sum += i;
    }

    println!("The arithmetic sum result is: {}", sum);
    if sum == 5050 {
        println!("Validation PASSED!");
        println!("============== APP 3: Completed Successfully ==============");
        0
    } else {
        println!("Validation FAILED!");
        println!("============== APP 3: Completed with Error ==============");
        -1 
    }
}
