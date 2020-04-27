pub fn fibonacci(size: i32) -> Vec<i32> {
    let mut prev = 1;
    let mut curr = 1;
    let mut array = vec![prev, curr];
    match size {
        0 => panic!("zero is not a right argument."),
        1 | 2 => array,
        _ => {
            let mut i = 3;
            while i <= size {
                let temp = prev + curr;
                array.push(temp);
                prev = curr;
                curr = temp;
                i += 1;
            }
            array
        }
    }
}