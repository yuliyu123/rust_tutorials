// x、y声明周期小于等于外部引用值声明周期
fn print_refs<'a, 'b> (x : &'a u32, y : &'b u32,) {
    println!("x = {}, y = {}", x, y);
}

fn failed_borrow<'a> () {
    let _x : i32 = 5;

    // violate "`_x` does not live long enough, borrowed value does not live long enough" rule
    // 要求_y生命周期为'a的生命周期，_x生命周期没有'a大
    // compile error
    // let _y : &'a i32= &_x;
}

#[test]
fn basic() {
    let (four, five) = (4, 5);
    
    print_refs(&4, &5);
    failed_borrow();
}


// 返回一个将输入和 `y` 相加的函数, 
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| {
        let sum = x + y;
        println!("sum: {}", sum);
        sum
     };
    closure
}


fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

#[test]
fn test_return_impl() {
    make_adder_function(1)(2);
}


// 长生命周期参数可以转换为短生命周期的参数，反之不行
// 'static 生命周期是可能的生命周期中最长的，它会在整个程序运行的时期中存在。'static 生命周期也可被强制转换成一个更短的生命周期。有两种方式使变量拥有 'static 生命周期，它们都把数据保存在可执行文件的只读内存区：
// 产生一个拥有 `'static` 生命周期的常量。
static NUM: i32 = 18;

// 返回一个指向 `NUM` 的引用，该引用不取 `NUM` 的 `'static` 生命周期，
// 而是被强制转换成和输入参数的一样。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

#[test]
fn static_basic() {
    {
        // 产生一个 `string` 字面量并打印它：
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 当 `static_string` 离开作用域时，该引用不能再使用，不过
        // 数据仍然存在于二进制文件里面。
    }

    {
        // 产生一个整型给 `coerce_static` 使用：
        let lifetime_num = 9;

        // 将对 `NUM` 的引用强制转换成 `lifetime_num` 的生命周期：
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}



// 就如泛型类型能够被约束一样，生命周期（它们本身就是泛型）也可以使用约束。: 字符的意义在这里稍微有些不同，不过 + 是相同的。注意下面的说明：

// T: 'a：在 T 中的所有引用都必须比生命周期 'a 活得更长。
// T: Trait + 'a：T 类型必须实现 Trait trait，并且在 T 中的所有引用都必须比 'a 活得更长。
use std::fmt::Debug; // 用于约束的 trait。

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` 包含一个指向泛型类型 `T` 的引用，其中 `T` 拥有一个未知的生命周期
// `'a`。`T` 拥有生命周期限制， `T` 中的任何*引用*都必须比 `'a` 活得更长。另外
// `Ref` 的生命周期也不能超出 `'a`。

// 一个泛型函数，使用 `Debug` trait 来打印内容。
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// 这里接受一个指向 `T` 的引用，其中 `T` 实现了 `Debug` trait，并且在 `T` 中的
// 所有*引用*都必须比 `'a'` 存活时间更长。另外，`'a` 也要比函数活得更长。
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}


#[test]
fn return_trait() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
