fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    // s.clear()会将s中的所有字符清空
    s.clear();

    let s = String::from("hello world");

    // 字符串slice是string中一部分值的引用 [starting_index..ending_index]
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // 如果索引从0开始，可以省略起始索引 [..2]和[0..2]相等
    let slice = &s[0..2];
    let slice = &s[..2];

    // 如果索引从末尾开始，可以省略结束索引 [..]
    let len = s.len();
    let slice = &s[3..];
    let slice = &s[3..s.len()];

    let first = first_word2(&s);
    println!("the first word is: {}", first);

    // 数组slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// 返回第一个单词的长度
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
