// crate 是一个Rust二进制文件或库文件，可供他人使用
// package 是一个或多个 crate，提供一系列的能力
// 一个package必须包含一个或0个库crate，但可以包含任意多个二进制crate，总体来说，必须包含至少一个crate（库或二进制crate）

// module可以让我们在一个crate里组织代码，也可以控制可访问性（public、private）
// 整个module树是挂在crate module下的
// 使用 mod 关键字来定义 module
// module里可以定义其他module
// Rust中，默认情况下，functions, methods, structs, enums, modules, and constants等都是私有的，父模块下的item无法访问该父模块的子模块下的私有item，但是子模块下的item可以使用祖先模块下的item

// src/lib.rs是库crate的根文件

mod front_of_house {
    // 使用 pub 让hosting可被外部module访问
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 使用super指代父模块
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// 库crate的公共 API
pub fn eat_at_restaurant() {
    // absolute path
    // 绝对路径调用
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // 相对路径调用
    front_of_house::hosting::add_to_waitlist();

    // 调用结构体的数据
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 调用枚举
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// 使用 use 就像是在创建一个链接，使得当前作用域下可以使用简短的名字去调用相应元素
// 使用 pub 可以重导出一个item，这样外部就可以使用这个元素了：`hosting::add_to_waitlist`
pub use crate::front_of_house::hosting;
// 使用 as 关键字进行重命名
use std::io::Result as IoResult;
// 可以使用一个花括号的方式同时引入一个模块或crate下的元素，这样就不用书写多行来引入了
use std::{cmp::Ordering, fs};

// 多维度引入
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// 通配符引入
use std::collections::*;

pub fn use_short_caller() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 使用分号结束一个 module 会告诉rust去加载这个module的实际内容
mod another_module;

use crate::another_module::module_hosting;

pub fn use_another_module_method() {
    module_hosting::add_to_waitlist();
}
