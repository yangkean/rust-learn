// 当 assertion 失败时会使用 debug 模式打印东西，所以需要实现 Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

// #cfg[(test)] 告诉 rust 只在执行 cargo test 的时候才编译和执行这段代码
// 单元测试一般跟你的代码放在一起，所以要使用这个标记来节省编译时间和编译体积
// 集成测试则是新建一个tests目录，该目录和src同级，然后在这个目录下新建测试文件，通过引用的方式引入模块进行测试，不需要使用这个标记，执行cargo test时rust会自动寻找这个目录
#[cfg(test)]
mod tests {
    // 使用外部模块的任何东西
    use super::*;

    // 使用 #[test] 使一个函数成为一个测试函数
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic] // 如果代码退出就符合预期
    fn another() {
        panic!("fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2), "看起来你的程序有问题哦，左{}, 右{}", 4, add_two(2));
    }
}
