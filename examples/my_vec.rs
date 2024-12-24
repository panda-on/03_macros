fn main() {
    let my_vec = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", my_vec);

    let my_vec = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", my_vec);

    let my_vec: Vec<usize> = my_vec![];
    println!("{:?}", my_vec);
}

/// my_vec! macro
#[macro_export]
macro_rules! my_vec{
    // my_vec![]
    () => {
        Vec::new()
    };
    // my_vec![1; 10]
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    // my_vec![1, 2, 3]
    ($($x:expr),*) => {
        {
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    }
}
