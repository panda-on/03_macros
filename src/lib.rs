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
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

// ? operator macro
#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        }
    };
}

// my_ready! macro
#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(v) => std::task::Poll::Ready(v),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
