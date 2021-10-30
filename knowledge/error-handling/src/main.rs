fn main() {
    // // 主动调用退出程序
    // panic!("crash and burn");

    // // 使用 `RUST_BACKTRACE=1 cargo run` 可以看到报错栈
    // let v = vec![1, 2, 3];

    // v[99];

    use std::fs::File;
    use std::io::ErrorKind;
    use std::io::Read;
    use std::io;
    use std::fs;

    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // 匹配不同的错误类型
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    // unwrap 会根据结果值决定是返回Ok的值还是调用panic!
    let f = File::open("hello.txt").unwrap();

    // panic 时会显示我们expect的信息，而不是使用默认的panic信息
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // // 返回错误给调用函数的调用方
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let f = File::open("hello.txt");

    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut s = String::new();

    //     match f.read_to_string(&mut s) {
    //         Ok(_) => Ok(s),
    //         Err(e) => Err(e),
    //     }
    // }

    // 使用 `?` 操作符简化错误抛出
    // 在Result后使用这个操作符，跟使用match是一样的，如果是个Ok,Ok内部的值会被返回，如果是个Err，这个Err会被当前函数返回给调用此函数的调用方
    fn read_username_from_file() -> Result<String, io::Error> {
        // let mut f = File::open("hello.txt")?;
        // let mut s = String::new();
        // f.read_to_string(&mut s)?;
        // Ok(s)

        // // 更简短的链式调用
        // let mut s = String::new();
        // File::open("hello.txt")?.read_to_string(&mut s)?;
        // Ok(s)

        // 更更简短的调用，这会完成全部操作，但会跳过一部分错误描述
        fs::read_to_string("hello.txt")
    }

    let username = read_username_from_file();
    println!("username is {:?}", username);
}
