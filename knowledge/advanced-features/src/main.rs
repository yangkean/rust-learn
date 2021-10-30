use std::ops::Add;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // 关联类型，必须要声明trait定义的关联类型
    // 同时，`type`也是类型重命名的一种语法，也可以用在方法里
    type Output = Point;

    // 重载add操作
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // 可以借用fmt::Display的方法to_string
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct TwoPoint {
    x: i32,
    y: i32,
}

impl OutlinePrint for TwoPoint {}

impl fmt::Display for TwoPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// 使用函数指针类型 `fn` 来定义一个函数参数
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let p = TwoPoint { x: 1, y: 3 };
    p.outline_print();

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}
