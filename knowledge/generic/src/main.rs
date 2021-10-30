struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 泛型定义函数参数
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

use generic::{Tweet, Summary, NewsArticle};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("integer.x = {}", integer.x());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    returns_summarizable(true);

    ///// 生命周期（lifetime）表示一个引用（reference）有效的范围
    // {
    //     let r;

    //     {
    //         let x = 5;
    //         r = &x;
    //     }

    //     println!("r: {}", r);
    // }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    
    // //// 生命周期注解语法
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

}

// 什么🐦语法，震惊劳资
// 'a 表示 x 和 y 有同样长的泛型生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// impl Trait 只能用于返回一种类型
fn returns_summarizable(switch: bool) -> impl Summary {
    // if switch {
    //     NewsArticle {
    //         headline: String::from(
    //             "Penguins win the Stanley Cup Championship!",
    //         ),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //         ),
    //     }
    // } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    // }
}
