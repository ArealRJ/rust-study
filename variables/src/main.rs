
fn main() {
    // 变量必须要加mut，不然不可以进行修改
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // const声明常量不可进行修改,并且必须带上类型
    // rust中常量命名的约定是单词之间使用_连接
    const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;


    // let和mut的区别在于，let相当于创建了一个新的变量，可以修改类型，并且复用变量名
    // let声明的变量在自己的作用域内有效，不会影响其他作用域
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    // √
    let spaces = "    ";
    let spaces = spaces.len();

    // × 会提示类型不匹配
    // let mut spaces = "    ";
    // spaces = spaces.len();


    /* 
     * rust中整形分为无符号（u）和有符号（i）
     * 无符号：表示从0开始的正整数 (0 - 2^n - 1) u8表示0-255
     * 有符号：可以表示负数 （-(2^(n - 1)) 到 2^(n - 1) - 1 ）i8表示-128-127
     */
    
    // 长度(bit)  有符号   无符号
    // 8           i8       u8
    // 16          i16      u16
    // 32          i32      u32
    // 64          i64      u64
    // arh         isize    usize

    // isize和usize类型依赖运行程序的计算机架构：32位系统是32，64位系统是64
    // 数字类型默认是i32

    // rust中的浮点类型只有f32和f64，默认是f64（精度更高，速度几乎一样）


    // rust中数学运算支持基本数学运算：+ - * / %
     
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    print!(
        "sum is {sum}, difference is {difference}, product is {product}, quotient is {quotient}, remainder is {remainder}"
    );

    /*
     * 布尔类型
     * rust中的布尔类型只有两个值：true和false
     */
    let t = true;
    let f: bool = false;


    /*
     * 字符类型：
     * rust中的字符类型只有一个值：char，是语言中最原生的字母类型
     * 注意：char类型使用单引号('')，大小为4个字节
     */

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    /*
     * 复合类型
     * rust中复合类型有两个：元组（tuple）和数组（array） 
     */

    // 元组类型：将多个其他类型的值合并进一个复合类型的主要方式，元组长度固定
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 元组取值，通过解构
    let (x, y, z) = tup;
    // 也可以通过(.)来根据索引访问
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // 数组类型
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [3; 5]; // [3, 3, 3, 3, 3] 表示数组长度为5，每个元素都是3
    // 数组的访问：通过索引方位
    // 注意：如果访问超过数组长度的元素，会导致panic错误
    let first = arr[0];
    let second = arr[1];

}
