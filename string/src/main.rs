fn main() {
    
    // String::from会请求一块内存
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);



    let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{},world",s1);

    let s2 = s1.clone();
    println!("s1 = {},s2 = {}",s1,s2);


    // i32存储在栈上，直接拷贝效率更高
    let x = 5;
    let y = x;
    println!("x = {},y = {}",x,y);

    // 如果想在函数调用完后继续使用s1，需要返回对应的值
    let (s3, len) = calculate_length(s1);
    
    let len = calculate_length2(&s3);

    println!("The length of '{}' is {}.", s3, len);

    let mut s4 = String::from("hello");
    calculate_length3(&mut s4);


    let mut s = String::from("hello");

    let mut s = String::from("hello");


    /*
      可变引用和不可变引用
     */
    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题

    // println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// 使用&来获取引用，所有权并不发生变化，这样的行为称为借用
// 借用来的变量不能进行修改
fn calculate_length2(s: &String) -> usize {
    s.len()
}

// &mut是可变引用，可变引用在一个作用域内，同时只能存在一个
// 可变引用和不可变引用不能同时存在
// 多个不可变引用可以同时存在
fn calculate_length3(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}


// 错误：在dangle执行时，创建了一个字符串s，返回了这个字符串的引用&s，当函数执行结束时，字符串s被销毁，此时&s成为悬垂指针
// 正确做法：直接返回字符串s而不是引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
      