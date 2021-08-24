/* 定义枚举 */
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 给枚举类型定义相关数据
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
// // 每个类型不易定义不同类型的数据
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// 枚举类型声明各种数据类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 声明枚举的方法
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    route(IpAddrKind::V4);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    // let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("v4 ip addr is {:?}, v6 ip addr is {:?}", home, loopback);

    let some_u8_value = Some(9);
    println!("value is {:?}", some_u8_value);
    // `if let` 相当于 match 只匹配一种情况时的语法糖
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("ip kind is {:?}", ip_kind);
}

fn value_in_cents(coin: Coin) -> u8 {
    // match 操作符比较到哪个分支，就会执行哪个分支的代码块
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn another(some_u8_value: u8) {
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // 当不想列举所有值的时候，可以用 `_` 其他所有值
        // () 表示啥都不会发生
        _ => (),
    }
}
