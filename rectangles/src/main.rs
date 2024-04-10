#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 关联函数
impl Rectangle {
    // 在impl块中，self是impl块类型的别名
    // 所有在impl中定义的函数被称为方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 不以self作为第一参数的关联函数（因此不是方法），这些函数通常称为new
    fn square(size: u32) -> Rectangle {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // let rect1 = (30,50);

    let scale = 2;
    let rect1 = Rectangle {
        // width: 30,
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        // area(rect1)
        area(&rect1)
    );

    // println!("rect1 is {:#?}", rect1)

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));


    // :: 语法用于关联函数和模块创建的命名空间
    let sq = Rectangle::square(20);
    println!("sq = {:#?}", sq);

}

// fn area(width:u32,height:u32)->u32{
//     width*height
// }

// 使用元组
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 使用结构体
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
