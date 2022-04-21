// unsafe: 提供了五个不会被编译器检查内存安全的功能:
// 解引用裸指针
// 调用不安全的函数或方法
// 访问或修改可变静态变量
// 实现不安全trait
// 访问union的字段
// 如果Rust不允许进行不安全操作，那么有些任务则根本完成不了。Rust 需要能够进行像直接与操作系统交互，甚至于编写你自己的操作系统这样的底层系统编程！

mod thread_lib;

extern crate core;

unsafe trait Foo {}

unsafe impl Foo for u32 {}

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

// 类似接口
pub trait HelloMacro {
    fn hello_macro();
}
pub struct Pancakes;

// HelloMacro实现自身hello_macro方法
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

// Pancakes::hello_macro();
