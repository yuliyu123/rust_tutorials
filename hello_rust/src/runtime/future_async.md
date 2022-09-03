
Rust Runtime 实现:

`Future trait`:

```Rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

- `Future` 通过 `poll` 被执行，返回 `output` 要么 `Pending` 要么 `Ready(T)`。
- `Context` 负责维护异步任务的上下文  
    - 目前只提供了 `Waker`，用于通知 `executor` 当前 `future` 已经准备好执行了

`poll` 方法被调用: 
1. 用户可以手动实现 Future和内部 poll方法
2. `Runtime` 调用

`Future` 的实现者，需要保证一旦返回了 `Poll::Pending`，就要在未来该 `Future` 依赖的 IO 就绪后能够唤醒它。唤醒一个 Future 是通过 Context 内的 Waker 做到的。至于说唤醒之后要做什么，这个由 Runtime 所提供的 cx 自行实现（如将这个 Task 加入到待执行队列）。

所以任何会产生事件的东西都要负责存储 Waker 并在 Ready 后唤醒它；而提供 cx 的东西会接收到这次唤醒操作，要负责重新调度它。这两个概念分别对应 Reactor 和 Executor，这两套东西靠 Waker 和 Future 解耦。

IO: 负责将 IO 注册到 Reactor, 使用 epoll 机制，没有则 wait().

Reactor: 存储 Waker 并在 Ready 之后唤醒(wake)任务

Executor: 实现`Future trait`, 提供 `ctx` 接受唤醒操作，并重新调度执行 `task`, `task` 其实是 `Future`

```Rust
pub struct Executor {
    local_queue: TaskQueue,
    pub(crate) reactor: Rc<RefCell<Reactor>>,

    /// Make sure the type is `!Send` and `!Sync`.
    _marker: PhantomData<Rc<()>>,
}
```

`Executor::block_on()` 执行 `local_queue` 的 `task`
将 `task` 包装为 `waker` 然后转为 `context` 传入 `Future` 的 `poll` 方法执行。


```Rust
 // consume all tasks
while let Some(t) = self.local_queue.pop() {
    let future = t.future.borrow_mut();
    let w = waker(t.clone());
    let mut context = Context::from_waker(&w);
    let _ = Pin::new(future).as_mut().poll(&mut context);
}
```

<!-- https://www.ihcblog.com/rust-runtime-design-1/#more -->
<!-- https://note.xuanwo.io/?utm_source=xuanwo&utm_medium=email#/page/rust%2Fasync%20runtime -->
