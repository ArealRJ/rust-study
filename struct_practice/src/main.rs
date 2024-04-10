fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    user1.email = String::from("someoneelse@example.com");


    // 使用结构体更新语法
    // 注意：此时user1的所有权已经转移给user2，因为user1的username已经被移动到user2中，user1无法使用，除非将user2.username重新赋值
    let mut user2 = User {
        email: String::from("someoneelse@example.com"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);


    let subject = AlwaysEqual;
}

/**
 * 结构体
 */
struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}

fn build_user(email:String,username:String)->User{
    User{
        active:true,
        // username:username, //名称相同可以使用简洁语法
        username,
        // email:email,
        email,
        sign_in_count:1
    }
}

/**
 * 元组结构体
 */

 struct Color(i32,i32,i32);
 struct Point(i32,i32,i32);


 /**
  * 类单元结构体
  */

 struct AlwaysEqual;