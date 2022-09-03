// 修改智能指针

use std::{rc::Rc, cell::RefCell, borrow::Borrow};

struct Foo {
    pub x: i32,
}

impl Foo {
    pub fn new(x: i32) -> Self {
        Foo { x }
    }

    pub fn add_one(&mut self) {
        self.x += 1;
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
}


#[test]
fn test_modify_ptr() {
    // Box ptr可以直接修改，因为他实现了Deref trait 和 DerefMut trait
    let mut ptr = Box::new(Foo { x: 1 });
    println!("{:p}", ptr);

    *ptr = Foo { x: 2 };
    println!("{:}", ptr.x);

    println!("-------------------");
    // 一个常见的组合就是 Rc 或者 Arc 和 RefCell, RefCell 提供了一种内部可变性， Rc默认不可变. 而这组合等同于C++ 的shared_ptr, shared_ptr也是可变的
    // 线程不安全的shared_ptr
    let rc = Rc::new(RefCell::new(Foo::new(1)));
    let s1 = rc.clone();
    let s2 = rc.clone();

    println!("rc: {:p}, s1: {:p}, s2: {:p}", rc, s1, s2);

    // 取到被管理对象的值，这里是 Foo 的实例化
    s2.borrow_mut().x = 2;
    // 地址不变，管理对象变
    println!("rc: {:p}, s1: {:p}, s2: {:p}", rc, s1.as_ptr(), s2.as_ptr());

    // 这里也要用borrow_mut() 获取内部可变性。 borrow() 返回不可变
    println!("x: {}", rc.borrow_mut().get_x());
    println!("x: {}", s1.borrow_mut().get_x());
    println!("x: {}", s2.borrow_mut().get_x());
    println!("count: {}", Rc::strong_count(&rc));


    // 线程安全的shared_ptr, Arc 和 RefCell 的组合
    println!("-------------------");
    let arc = Rc::new(RefCell::new(Foo::new(1)));
    let s1 = arc.clone();
    let s2 = arc.clone();

    s2.borrow_mut().x = 3;
    println!("arc: {}", arc.borrow_mut().get_x());
    println!("s1: {}", s1.borrow_mut().get_x());
    println!("count: {}", Rc::strong_count(&arc));
}
