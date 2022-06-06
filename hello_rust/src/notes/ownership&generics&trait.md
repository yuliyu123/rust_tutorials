## 所有权
每个变量都有所属作用域，当超过作用域就会销毁其所占堆栈空间；
变量默认不可变，可读不可写，需要改变要加mut修饰符；
含有引用和借用类型，默认赋值方式为浅拷贝加移动的形式(类似于move语义)。前者所使用的变量就会不再可用，后面销毁的时候就会释放最新的变量占用空间(防止double free)。
要深拷贝需要使用clone方法；
可变和不可变两种声明不能在同一作用域使用，目的为防止条件竞争、脏读等现象；

let r = &x
r是x的引用，x是r的借用。

note: 借助所有权、引用、借用等规则在编译期可以预防内存泄露、野指针、条件竞争等风险，并且不需要gc, 实现zero-cost内存管理开销，缺点是降低开发速度，使用者需要熟悉其语法规则。

## 泛型（generic type）
和C++泛型没啥区别，编译器展开插入具体类型，运行期无消耗，即单态化过程。
rust也可实现struct方法，只是在struct外部实现。

	impl<T, U> Point<T, U> {
	    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
	        Point {
	            x: self.x,
	            y: other.y,
	        }
	    }
	}

## trait
trait 和 trait bound 让我们使用泛型类型参数来减少重复, 需要实现泛型某些特定方法，例如PartialOrd + Copy等。

实现泛型T的PartialOrd和Copy trait, 并且bound.
	fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

当使用trait对象时，Rust必须使用动态分发, 即在运行期使用trait对象中的指针来知晓需要调用哪个方法。
只有对象安全（object safe）的trait才可以组成trait对象。
一个 trait 中所有的方法有如下属性时，则该 trait 是对象安全的：
返回值类型不为 Self
方法没有任何泛型类型参数


## 生命周期
int, string这些常用数据类型一开始会知道所使用栈空间大小，默认copy形式。vec、hashmap这些可变容器大小不会copy，会动态分配，循环遍历为引用方式。
生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
被编码进Rust引用分析的模式被称为生命周期省略规则（lifetime elision rules）。这并不是需要开发者遵守的规则；这些规则是一系列特定的场景，此时编译器会考虑，如果代码符合这些场景，就无需明确指定生命周期。

trait和trait bounds 保证了即使类型是泛型的，这些类型也会拥有所需要的行为。由生命周期标注所指定的引用生命周期之间的关系保证了这些灵活多变的代码不会出现悬垂引用。
