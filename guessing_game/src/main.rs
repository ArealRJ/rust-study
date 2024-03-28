use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("secret_number is {secret_number}");

    // println!("Please input your guess.");

    loop {
        println!("Please input your guess.");

        // String::new()调用String类型上的构造方法，返回一个新的空字符串
        let mut guess = String::new();

        // &mut guess和&guess的区别：
        // &mut guess是可变引用，&guess是不可变引用
        // 简单理解为&mut guess是可写引用，&guess是不可写引用
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // 将guess转换为u32类型，如果转换失败，则继续循环
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("You guessed:{guess}");

        // 调用cmp方法，进行比较
        match guess.cmp(& secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
        },
        }
    }




}
