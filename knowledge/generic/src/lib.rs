// trait 是给一个特别的类型定义拥有的功能以及可以跟其他类型共用(share)的能力。我们可以使用抽象的方式给 trait 定义共用(shared)行为
// trait 可以类比其他语言中的接口，不过有一些区别（我觉得更像抽象类

pub trait Summary {
  fn summarize(&self) -> String;

  // // 定义一个默认的实现而不是抽象签名
  // fn summarize(&self) -> String {
  //   String::from("(Read more...)")
  // }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// 用 for 关键字指定我们想实现相应 trait 的类型
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}

// impl Summary for Tweet {}

// 使用 Trait 作为参数，是 trait bound 的语法糖
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// pub fn notify(item: &(impl Summary + Display)) {
//   //
// }

// // trait bound
// pub fn notify<T: Summary>(item: &T) {
//   println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary + Display>(item: &T) {
//   //
// }

// // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// // 使用 where 分句避免函数签名杂乱
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {
//   //
// }

// 返回类型
fn returns_summarizable() -> impl Summary {
  Tweet {
      username: String::from("horse_ebooks"),
      content: String::from(
          "of course, as you probably already know, people",
      ),
      reply: false,
      retweet: false,
  }
}
