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

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}


// test
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if (2+2 == 4) {
            Ok(())
        } else {
            Err(String::from("two add two is not equal four"))
        }
    }
}

