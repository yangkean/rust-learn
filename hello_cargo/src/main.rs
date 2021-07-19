// Rust 是一种 提前编译语言（ahead-of-time compiled language），同时有类型推断
// 如果文件名包含多个单词，使用下划线分隔它们
// main 函数是特殊的：它是每个可执行的 Rust 程序首先执行的
// Rust 要求所有函数体都要用花括号包裹起来
// Rust 使用 4 个空格的缩进风格
fn main() {
    // 当看到符号 ! 的时候，调用的是宏而不是普通函数
    println!("Hello, world!");
}

// 关于编译：
// 可以通过 rustc 命令来使用 Rust 编译器，并传递源文件的名字给它
// $ rustc main.rs
// 但对于大项目来说更好的方法是使用 Rust 的构建系统和包管理工具 Cargo。Cargo 负责构建代码、下载依赖库并编译他们
// 使用 Cargo 创建项目：
// 进入目录并执行：
// $ cargo new hello_cargo --bin
// `hello_cargo` 是项目名称，`--bin` 参数告诉 Cargo 创建一个二进制项目，而不是一个库
// 编译应用（构建项目）：$ cargo build
// 编译并执行应用：$ cargo run
// 注意：Cargo 如果发现文件并没有被改变，就会直接运行二进制文件。如果修改了源文件，Cargo 会在运行之前重新构建项目
// 发布项目：$ cargo build --release # 这个命令会优化编译的项目，使程序运行地更快，不过编译会慢点，开发时为了快速迭代快速测试应使用 run 命令
// Cargo 官网：http://doc.crates.io/guide.html
