use macros::EnumFromDarling;

fn main() {
    let left: Direction<i32> = 10.into();
    let up: Direction<i32> = DirectionUp::new(10).into();
    println!("{:?}, {:?}", up, left);
}

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}
