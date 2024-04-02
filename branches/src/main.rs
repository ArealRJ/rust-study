fn main() {
    let number = 3;

    if  number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // rust中代码块的值是最后一个表达式的值
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {number}");

    // 错误:类型不一致
    // let number = if condition { 5 } else { "six" };

    /*
     rust 循环包含loop while for
     */

    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("loop:{counter}");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // 循环标签
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1; 
    }

    println!("End count = {count}");


    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    let a = [1, 2, 3, 4, 5];
    let mut index = 0;
    while  index < 5 {
        println!("while--the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("for--the value is: {element}");
    }


    for number in (1..4).rev()  {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");

}
