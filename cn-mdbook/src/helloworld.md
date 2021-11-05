# Hello, world

有多少种 Hello world 的写法？

前置知识：无  
练习地址：[https://play.rust-lang.org](https://play.rust-lang.org)  

```rust, editable
fn main() {

    println!("Hello, world!");
    println!("{}", "Hello, world!");

    let name = "world";
    println!("Hello, {}!", name);
    println!("{}", format!("Hello, {}!", name));

    let word1 = "Hello";
    let word2 = "world";
    let word3 = word1.to_owned() + ", " + word2 + "!";
    println!("{}", word3);

}
```