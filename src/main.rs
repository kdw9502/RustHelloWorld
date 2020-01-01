fn main() {
    let text = "Hello, world!";
    let mut mutable_text: &str = "";
    mutable_text = text;
    println!("{0}", mutable_text);
    //https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

    let var: u64 = "452".parse().unwrap();
    let func = func1;
    print!("{}",func(1));
}

fn func1(a:i32) -> i32
{
    6
}
