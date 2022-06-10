// unsafe: 提供了五个不会被编译器检查内存安全的功能:
// 解引用裸指针
// 调用不安全的函数或方法
// 访问或修改可变静态变量
// 实现不安全trait
// 访问union的字段
// 如果Rust不允许进行不安全操作，那么有些任务则根本完成不了。Rust 需要能够进行像直接与操作系统交互，甚至于编写你自己的操作系统这样的底层系统编程！

pub mod ds;
pub mod traits;
pub mod concurrency;
pub mod lifetime;
pub mod demos;
pub mod fs;

#[allow(dead_code)]
fn test_atti() {
    println!("dead code");
}
