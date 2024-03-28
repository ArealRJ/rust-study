// main函数，是程序的入口 
// 在可执行的rust程序中，main函数总是最先运行的函数
fn main() {
  // !代表调用宏，而不是普通函数
  println!("Hello, world!");
}

// 执行rust文件步骤
// 编译 rustc main.rs 
// 生成二进制可执行文件
// .exe是windows下的可执行文件后缀
// .pdb是windows下的调试文件后缀