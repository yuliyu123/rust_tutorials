use std::cmp::Ordering;

pub fn bin_search(arr: Vec<i32>, x: i32) -> i32 {
    let (mut left, mut right) = (0, arr.len() - 1);
    loop {
        if left > right {
            return -1;
        }

        let mut mid = (left + right) / 2;
        match arr[mid].cmp(&x) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid - 1,
            Ordering::Equal => {
                while mid >= 1 && arr[mid - 1] == arr[mid] {
                    mid -= 1;
                }

                return mid as i32;
            }
        }
    }
}
