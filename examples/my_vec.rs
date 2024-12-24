use macros::my_vec;

fn main() {
    let my_vec = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", my_vec);

    let my_vec = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", my_vec);

    let my_vec: Vec<usize> = my_vec![];
    println!("{:?}", my_vec);
}
