use std::collections::LinkedList;

fn main() {
    let list1 = LinkedList::from([1, 2, 3, 4]);
    let list2: LinkedList<_> = [1, 2, 3, 4].into();
    assert_eq!(list1, list2);
}
