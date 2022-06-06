use std::collections::LinkedList;

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// approach 2
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    // 容器大小不会copy，会动态分配，循环遍历为引用方式
    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    // 返回list中最大值的引用，这里list不会销毁
    &largest
}

#[test]
fn greatest_test() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    assert_eq!(*result, 'y');
}

#[derive(Debug)]
pub struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[test]
fn from_into_trait() {
    // from(x) === x.into(), convert from x type into y type.
    let list1 = LinkedList::from([1, 2, 3, 4]);
    let list2: LinkedList<_> = [1, 2, 3, 4].into();
    assert_eq!(list1, list2);

    let i = 5;
    let num: Number = i.into();
    println!("{:?}", num);
}

#[test]
fn from_into_trait2() {
    // allocate on stack
    let str = "str";
    // on heap
    let Str = String::from(str);
    assert_eq!(str, Str);
}


fn some_number() -> Option<u32> {
    Some(42)
}


fn age() -> u32 {
    15
}

#[test]
fn bound_test() {
    match some_number() {
        // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
        Some(n @ 42) => println!("The Answer: {}!", n),
        // 匹配任意其他数字。
        Some(n)      => println!("Not interesting... {}", n),
        // 匹配任意其他值（`None` 可变类型）。
        _            => (),
    }

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // 可以直接匹配（`match`） 1 ..= 12，但那样的话孩子会是几岁？
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结果。
        n             => println!("I'm an old person of age {:?}", n),
    }
}
