use foba::number_vec_collection::fibonacci;
use foba::number_vec_collection::range;

#[test]
fn test_fibo() {
    let some = fibonacci(8);
    for (i, x) in some.iter().enumerate() {
        println!("[{}] ({})", i, x)
    }
}

#[test]
fn test_range() {
    let some = range(8);
    for (i, x) in some.iter().enumerate() {
        println!("[{}] ({})", i, x)
    }
}