use crate::container::list::List;

mod container;

fn main() {
    let mut list: List = List::new();

    for i in 0..10 {
        list.push(i);
    }
    list.pop();
    list.debug();
}
