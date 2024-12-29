use macros::AutoDeref;

#[allow(unused)]
#[derive(Debug, AutoDeref)]
#[deref(field = "inner")]
pub struct RespBulkString {
    inner: String,
    nothing: (),
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };
    // 使用解引用来访问 `inner` 的值
    println!("Inner via deref: {:?}", *s); // 使用 Deref 转换为 &String
}
