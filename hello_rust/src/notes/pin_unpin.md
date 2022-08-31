## pin
是一个智能指针类型，确保被它包裹的指针指向的数据不会被 move。

只有一种情况例外：自引用类型. 最常见的自引用类型是实现了 `Future` 的对象

pin!: 数据 pin 在栈上，不会有堆分配

Box::pin(): 将数据 pin 在堆上

futures::pin_mut: 数据 pin 在栈上

使用: `Pin<&mut Self>` 用于保证 `Self` 不被 `move`

## unpin

实现了pin的字段可以move. Rust 中的大部分类型都可以安全的 move，标准库默认为他们实现了 Unpin。


https://note.xuanwo.io/?utm_source=xuanwo&utm_medium=email#/page/rust%2Fstd%2Fpin
https://note.xuanwo.io/?utm_source=xuanwo&utm_medium=email#/page/rust%2Fstd%2Ffuture