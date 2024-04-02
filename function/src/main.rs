// main函数
fn main() {
    println!("Hello, world!");
    
    // 调用函数
    another_function(5);

    print_labeled_measurement(5, 'h');

    // 错误，rust中复制语句没有返回值 rust代码是由表达式组成的
    // let x = (let y = 6)


    
    let y = {
        let x = 3;
        x + 1 // 表达式结果没有分号
    };

    println!("The value of y is: {y}");


    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

// 在函数签名中必须生命每个参数的类型
fn another_function(x: i32) {
    println!("the value of x is:{x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 函数返回值，使用return关键字，大部分函数隐式的返回最后的表达式
fn five() -> i32 {
    5
}

fn plus_one(x:i32) -> i32 {
    // x + 1; 错误：加上分号表示一个语句，而不是表达式
    x + 1
}