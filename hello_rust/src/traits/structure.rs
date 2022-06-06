
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
