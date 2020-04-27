use foba::vec_collection::number_vec_collection::fibonacci::fibonacci;

#[test]
fn test() {
    let some = fibonacci(8);
    for (i, x) in some.iter().enumerate() {
        println!("[{}] ({})", i, x)
    }
}