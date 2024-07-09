fn first_element<T>(list: &[T]) -> Option<&T> {
    let first = &list.first().unwrap();
    Some(first)
}

fn last_element<T>(list: &[T]) -> Option<&T> {
    match &list.last() {
        Some(val) => return Some(val),
        None => None,
    }
}

fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let first_slice: Option<&i32> = first_element(&arr);
    if let Some(val) = first_slice {
        println!("{val}");
    }

    let last_slice: Option<&i32> = last_element(&arr);
    if let Some(last) = last_slice {
        println!("{last}");
    }
}
