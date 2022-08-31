互斥锁(Mutex): Mutex<T>控制只能有单独一个线程读取/修改对象。通常在外面加上原子引用计数Arc变成Arc<Mutex<T>>，来减少Mutex拷贝的开销。对于多读少写的场景，可以用RwLock提高并发。
条件变量(CondVar) 条件变量用于“阻塞”线程并使得线程在等待事件时不需要消耗 CPU 时间。通常会与放进互斥锁、布尔型的预言值（状态值）关联使用，在状态值发生变化时通知条件变量。
屏障(Barrier) 屏障用于在某个需要若干线程都完成前置操作后再开始计算的操作之前，让所有所需线程的状态都达到能开始进行计算的状态。


Box<dyn ...>: 类似unqiue_ptr, 独占权，非共享ptr，无额外开销，分配内存在堆上。
Rc: 类似C++的shared_ptr, 本身count计数不能保证原子性，因此非线程安全。解决弱引用使用Weak<T>不具有所有权，(仅记录被引用对象是否还存在)，被管理对象不可变.作为单线程场景下的引用计数指针.
Arc<T>: 实现原子计数，线程安全。
Cell<T>: 提供了一种内部可变的机制。可以通过&self修改内部的值，而无需通过&mut self。零开销，但修改只能整体地修改，不能通过&self拿到内部的&mut T。
RefCell: Rc管理对象默认不可变, RefCell提供了一种内部可变的机制，需要使用RefCell改变其管理对象内容，二者常搭配使用。


Deref：解引用语义，结合Rust中的自动解引用机制，可允许自定义包装类型如智能指针等变量像内部变量一样使用，但使用仅限于取&self，如果要取&mut self则需要DerefMut这个trait。
Drop：析构函数。在变量生命周期结束时将自动执行drop以销毁。
Any：提供了一种简单的动态反射机制，要求类型必须具有'static的生命周期。在运行时可以downcast到任意类型，但若实际类型与要转换的类型不一致时，将返回Err。


Option: 表示可有值，可无值(None)。
Some: 有值。


Box<dyn(Fn() + Send + 'static)>: 并发用。
Box -> 来包裹dyn生成的智能指针对象，分配在堆上。
'static -> 静态生命周期，因为在多线程的时候很可能主线程结束了，多线程仍然在跑，没有'static的话子线程生命周期会随着主线程drop。放在全局只读区，生命周期长。
Send —> 可以把其ownership安全传递到其他线程中。
Sync -> 可以在线程中安全共享数据。
Fn() -> 用&self当做参数。
dyn -> 编译期不知道大小，需要动态派发.

maker.rs四个特性：

Send Sync Sized Copy

Sized -> 在编译期可确定大小的类型
Unsized -> 动态大小类型，在编译期无法确定其大小
?Sized -> 包含Sized和Unsized所标识的两种类型
https://skyao.io/learning-rust/std/marker/sized/rust-bczd.html

clone&copy:
clone是copy的supertrait,所copy类型也一定是clone类型, 类似于C语言memcpy。
copy仅只对栈内存做按位复制，而不对任何堆内存负责。所以String::new()、Vec<T>和Box<T>这种分配在堆上的对象无法使用copy，只能用clone进行深拷贝，copy通常用于原生类型及其组合类型（结构体、元组、枚举等）。
clone可以用户自定义行为，copy不行。

future: 代表一个任务，目前还没有完成，未来会完成。此时线程可以执行其他任务，等待有结果返回时再去执行该任务结果。
async/await: 联合使用。异步编程，使线程不会处于阻塞中，当有结果时再去执行结果。否则执行其他任务，提高线程利用率。
async Return a Future instead of blocking the current thread.

C++ future and promise:
promise: 保存T类型值，该值可被future对象通过get_future()读取。此时两个对象共享相同的共享状态，并且Promise对象是异步provider，它可以在某一时刻设置共享状态的值。Future对象可以返回共享状态的值。
future可以转移ownership给另一个线程，此时promise可以作为多线程同步的一种机制。
https://blog.csdn.net/jiange_zh/article/details/51602938
