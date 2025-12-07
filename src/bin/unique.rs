fn unique(mut a: Vec<i32>) -> Vec<i32> {
    a.sort();
    a.dedup();
    a
}

fn unique_generic<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    a.sort();
    a.dedup();
    a
}

fn main() {
    let input = vec![1, 2, 2, 3, 3];
    let output = unique(input);
    println!("{:?}", output);

    let input_generic = vec![1, 2, 2, 3, 3];
    let output_generic = unique_generic(input_generic);
    println!("{:?}", output_generic);

    let input_string = vec!["a", "b", "b", "c", "c"];
    let output_string = unique_generic(input_string);
    println!("{:?}", output_string);
}

#[test]
fn empty_list() {
    let input = vec![];
    let output = unique(input);
    assert_eq!(output, vec![]);
}

#[test]
fn sorted_list() {
    let input = vec![1, 2, 3];
    let output = unique(input);
    assert_eq!(output, vec![1, 2, 3]);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 3, 2];
    let output = unique(input);
    assert_eq!(output, vec![1, 2, 3]);
}

#[test]
fn unsorted_with_duplicates() {
    let input = vec![1, 3, 3, 2, 5, 5, 4, 6];
    let output = unique(input);
    assert_eq!(output, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn sorted_with_duplicates() {
    let input = vec![1, 2, 2, 3, 3];
    let output = unique(input);
    assert_eq!(output, vec![1, 2, 3]);
}