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
      