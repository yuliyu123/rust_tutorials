use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn ChannelThread() {
    // multiple producer single consumer
    let (tx, rx) = mpsc::channel();

    // 派生子线程，子线程中的值连同所有权发送到主线程
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
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
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
            // 这里不能通过，因为子线程已将val通过channel发送出去，不再拥有ownership
            // println!("val is {}", val);
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // 主线程获取val的ownership. recv()同步阻塞。try_recv()不非阻塞
    // let received = rx.recv().unwrap();
    // println!("{}", received)
}

// Arc(atomically reference counted)->原子引用计数, 线程安全的方式改变引用计数的类型.
// Rc(reference counted)->智能指针创建引用计数的值.
// Rc<T>并不能安全的在线程间共享,没有使用任何并发原语,来确保改变计数的操作不会被其他线程打断
// Mutex智能指针对互斥锁的封装,管理mutex生命周期
// Arc<T>包装一个Mutex<T>能够实现在多线程之间共享所有权
pub fn shared_status() {
    // let counter = Rc::new(Mutex::new(0));  // 非线程安全，不能通过
    // 线程安全改变引用计数，原子性
    let counter = std::sync::Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
