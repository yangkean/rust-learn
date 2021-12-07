#[derive(Debug)] // 此注解为结构体显式选择 `Debug` 功能
struct Rectangle {
    length: u32,
    width: u32,
}

// 这是一个 impl（implementation 的缩写）块
// 在 impl 里实现结构体的方法
impl Rectangle {
    // 方法与函数类似，但方法是在结构体的上下文中被定义的，并且方法的第一个参数总是 self，它代表调用该方法的结构体的实例
    fn area(&self) -> u32 {
        self.length * self.width
    }

    // impl 块中还可以定义不以 self 为参数的函数，称为关联函数（associated functions），它与结构体相关联，关联函数并不作用于结构体的实例上，可以使用结构提名和 `::` 语法来调用关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

// 每个结构体允许拥有多个 impl 块
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    /* 定义并实例化结构体 */

    // 结构体（structure），是一个允许我们命名并将多个相关值包装进一个有意义的组合的自定义类型。使用 struct 关键字定义
    // 结构体的每一部分可以是不同类型，每一部分都需要命名
    #[derive(Debug)]
    struct User {
        username: String, // 定义时定义名字（字段）和字段类型
        email: String,
        sign_in_count: u64,
        active: bool,
        hh: (i32, i32)
    }
    // 定义结构体后通过为每个字段指定具体值来创建该结构体的实例
    // 构建结构体实例时若字段名与变量名（字段值是个变量）同名，则可以简写为一个字段名
    let mut user1 = User {
        email: String::from("some@example.com"),
        username: String::from("some123"),
        active: true,
        sign_in_count: 1,
        hh: (32, 32)
    };
    // 结构体更新语法：利用 `..` 指定未显示设置的字段应有与给定实例对应字段相同的值
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another123"),
        ..user1
    };

    user1.email = String::from("another@example.com");
    println!("user1 is: {:?}, user2 is: {:?}", user1, user2);

    // 元组结构体（tuple structs）：与元组相像的结构体，有着结构体名称提供的含义，但没有具体的字段名只有字段的类型
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /* 一个使用结构体的实例程序 */

    let rect1 = Rectangle { length: 50, width: 30 };
    println!("The area of the rectangle is {} square pixels", area(&rect1));
    // 在 `{}` 中加入 `:?` 指示符告诉 `println!` 我们想要使用 `Debug` 的输出格式，`Debug` 是一个 trait，它允许我们在调试代码时以一种对开发者有帮助的方式打印出结构体
    println!("rect1 is {:?}", rect1);

    /* 方法语法（method syntax） */

    let rect2 = Rectangle { length: 50, width: 30 };
    println!("The area of the rect2 is {} square pixels", rect2.area());

    let rect3 = Rectangle { length: 40, width: 10 };
    let rect4 = Rectangle { length: 45, width: 60 };
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));

    println!("The square is {:?}", Rectangle::square(40));
}

// 计算长方形面积
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
