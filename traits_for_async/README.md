# Traits for Async (异步主要的 Traits)

## Future trait

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

- `cx`: 参数及其`Context`类型是运行时实际知道何时检查任何给定`future`的关键，同时仍然保持惰性
- `self`的类型注解与其他函数参数的类型注解类似，但有两个关键区别：
    - 它告诉Rust调用该方法时`self`必须是什么类型
    - 它不能是任意类型：它被限制为方法实现的类型、对该类型的引用或智能指针，或者是包装该类型引用的`Pin`

### Pin

- `Pin`是针对(类)指针类型(如&, &mut, Box和Rc)的包装器
    - 从技术上讲，`Pin`适用于实现`Deref`或`DerefMut`特性的类型，但这实际上等同于只适用于指针
- `Pin`本身不是指针，也没有像`Rc`和`Arc`那样具有引用计数等自身行为
- 它纯粹是编译器可以用来强制约束指针使用的工具

### Pin & Unpin

- `Future`中的一系列`await`点被编译成一个状态机，编译器确保该状态机遵循Rust所有关于安全性的常规规则，包括借用和所有权
- Rust会查看在一个`await`点和下一个`await`点或异步块结束之前需要哪些数据
    - 然后它在编译后的状态机中创建响应的变体
    - 每个变体都获得它所需的访问权限，以访问在源代码那部分中将要使用的数据
- 如果我们在给定的异步块中出现任何关于所有权或引用的错误，借用检查器会告诉我们
- 但当我们想要移动对应于该块的`Future`时(比如将其放入Vec中后传递给`join_all`)，事情就会变得复杂
- 当我们移动一个`Future`时，这就意味着移动(Rust为我们创建的)状态机
- 与Rust中的大多数其他类型不同，Rust为异步块创建的`Future`可能最终在任何给定变体的字段中**包含对自身的引用**
- 默认情况下，任何具有对自身引用的对象移动起来都不安全
- 如果你移动数据结构本身，那些内部引用将指向旧位置，该内存位置现在是无效的
- 一方面，当你对数据结构进行更改时，其值将不会更新
- 另一方面，更重要的是：系统可以将该内存自由的重用于其他目的。你可能最终会读取完全不相关的数据
- 理论上，Rust编译器可以尝试在对象每次移动时，更新对它的每个引用，但这可能会增加很多性能开销，特别是如果需更新整个引用网络
- 如果我们能够确保所讨论的数据结构不会在内存中移动，就不必更新任何引用
    - 这正是Rust的借用检查器所要求的：在安全代码中，它防止你移动任何有活跃引用的项目
- `Pin`基于此给我们提供了我们所需的确切**保证**
    - 当我们通过将指向该值的指针包装在`Pin`中来固定一个值时，它不能再移动
    - 因此，如果你有`Pin<Box<SomeType>>`,你实际上是固定了`SomeType`值，而不是`Box`指针
- `Box`指针仍然可以自由移动
    - 我们关心的是确保最终被引用的数据保持在原位。如果指针四处移动，但它指向的数据在同一位置，就没有潜在的问题
    - 关键是自引用类型本身不能移动，因为它仍然被固定

### Unpin & !Unpin

- 大多数类型即使在`Pin`指针后面，也完全可以安全的移动
    - 我们只需要在项目有内部引用时考虑固定
- 原始值(如数字和布尔值)是安全的。在Rust中，你通常使用的大多数类型也没有
- 我们需要一种方法告诉编译器，在这种情况下移动项目是可以的，这就是`Unpin`发挥作用的地方
- `Unpin`是一个标记特性(marker trait)，它本身没有功能
    - 标记特性的存在只是为了告知编译器，在特定上下文中使用实现给定`trait`的类型是安全
    - `Unpin`通知编译器，给定类型不需要维持关于“这个值是否可以安全移动”的任何保证。
- 就像`Send`和`Sync`一样，编译器会自动为所有可能证明安全的类型实现`Unpin`

### !Unpin

- 一个特殊情况，是没有为某些类型实现`Unpin`的情况
- 这种情况的表示法是`impl !unpin for SomeType`
- 当一个指向该类型的指针被包裹在`Pin`中使用时，这个`SomeType`类型就必须维持这些保证(也就是"不能被移动"的保证)
  ，才能确保使用时的内存安全

### Pin和Unpin之间的关系

- `Unpin`是"正常"情况，而`!Unpin`是特殊情况
- 一个类型是实现`Unpin`还是`!Unpin`，只有当你使用指向该类型的固定指针(如`Pin<&mut SomeType>`)时才重要(有关系)

## Stream trait

- `Stream`类似异步的`iterator`，暂时在`std`中没有定义
- 在`futures crate`里面有一个比较常见的定义

### 回顾 Iterator 和 Future trait 的定义

- 从`Iterator`中，有序列的概念：
    - 它的`next`方法提供一个`Option<Self::Item>`
- 从`Future`中，有随时间就绪的概念：
    - 它的`poll`方法提供一个`Poll<Self::Output>`

### Stream trait

为了表示随时间变得就绪的项目序列，定义了将这些特性整合在一起的`Stream trait`:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}
```

- `Item` 关联类型：用于流产生的项目类型
    - 可有0个或多个项目
- `poll_next`方法：可获取这些项目
    - 像`Future::poll`一样轮询
    - 像`Iterator::next`一样产生一系列项目
    - 返回`Poll<Option<Item>>>` 
