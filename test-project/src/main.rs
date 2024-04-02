fn main() {
    let result = fibonacci(10);
    println!("{result}")
}


// 斐波那契
fn fibonacci(n: i32) -> i32 {
    if n < 2{
        n
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}


// 摄氏度转华氏度
fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}