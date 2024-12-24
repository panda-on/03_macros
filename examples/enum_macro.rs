use macros::EnumFrom;

fn main() {
    let left: Direction = 10.into();
    let up: Direction = DirectionUp::new(10).into();
    println!("{:?}, {:?}", up, left);
}

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}

// manually impl
// impl From<u32> for Direction {
//     fn from(v: u32) -> Self {
//         Self::Left(v)
//     }
// }

// impl From<DirectionUp> for Direction {
//     fn from(v: DirectionUp) -> Self {
//         Self::Up(v)
//     }
// }
