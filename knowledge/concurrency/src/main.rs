use std::thread;
use std::time::Duration;
use std::sync::mpsc; // mpsc 表示“multiple producer, single consumer”
use std::sync::{Arc, Mutex};

fn main() {
    // 新线程执行，传入闭包函数在新线程中去执行代码
    // spawn 返回的是一个 JoinHandle 类型的值，当我们对这个值调用 join 方法，将会把它的线程执行完
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // 停止线程的执行
            thread::sleep(Duration::from_millis(1));
        }
    });

    // // 改变执行顺序
    // handle.join().unwrap();

    // 主线程执行
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 在主线程退出前执行完剩下的线程任务
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    // 用move夺取所有权，放置变量失效
    let another_handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    another_handle.join().unwrap();

    // 创建通道
    // tx 是发送端，rx是接收端
    let (tx, rx) = mpsc::channel();
    // 使用move夺取 tx所有权
    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap();

        // 发送多次消息
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // // recv 会阻塞主线程直到接收到信息，try_recv方法不会
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // 将 rx 作为一个 iterator，通道关闭后轮询就会结束
    for received in rx {
        println!("Got: {}", received);
    }

    println!("start multiple producers");
    // 多线程发送消息
    let (txm, rxm) = mpsc::channel();
    let txm1 = txm.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            txm1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            txm.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rxm {
        println!("Got: {}", received);
    }

    // 互斥器（Mutex）只允许一个线程访问数据，而且要提前获取锁来请求访问数据
    let m = Mutex::new(5);
    {
        // lock 方法会返回一个智能指针
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
