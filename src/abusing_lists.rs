#[allow(unused)]
fn abusing_lists() {
    let mut list = vec![1, 2, 8, 8, 1];

    for (i, num) in list.iter_mut().enumerate() {
        if *num % 2 == 0 {
            // list.remove(i);
        }
    }
}
