* rust不会批准有空值。
* int, string这些常用数据类型一开始会知道所使用栈空间大小，默认copy形式。vec、hashmap这些可变容器大小不会copy，会动态分配，循环遍历为引用方式。
生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
* use super::*; -> 宏定义
#[cfg(test)] attribute，标注告诉Rust只在执行 cargo test时才编译和运行测试代码
#[test] ->标注为test case
#[ignore] -> 标注为skip case, 暂时不执行.

* 函数默认私有，并且测试可对私有函数进行测试。公有：pub. mod类似于interface的定义，并且有访问权限，默认为private, 需要声明为pub才能供外部使用。
* crate 是一个二进制项或者库
* 末尾加分;表示语句，不会作为返回值。不加;会作为返回值返回
* match类似于case
* Result<T, E>可回复错误；不可恢复panic!()
* 传播错误的简写：? 运算符, 表示出错了直接返回，可在链式调用中使用。和Result联合使用。

		fn read_username_from_file() -> Result<String, io::Error> {
		    let mut s = String::new();

		    File::open("hello.txt")?.read_to_string(&mut s)?;

		    Ok(s)
		}

* 数据和行为分开，struct和enum不作为对象看待。



* 宏
println!;
println宏的名称
!表示它是一个宏。
panic!宏：https://doc.rust-lang.org/stable/std/macro.panic.html
#[derive(...)]: 派生宏。


# collections
hashmap(key无序) bshashmap(key有序)
hashset(key无序) bshasset(key有序)
linkedlist 双向，可作为栈(push_front())和队列(push_back())
