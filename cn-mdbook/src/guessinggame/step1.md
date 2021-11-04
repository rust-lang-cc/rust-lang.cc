
# Guessing game

Step 1

目标：获取玩家输入的数字并打印出来

<br>
<details>
    <summary>查看提示</summary>

目标：导入 std::io ，写出 fn main(), 提示输入 guess number 并打印玩家输入的数字。

下面有答案。请尽量自己写出来（或复习 [The book 第二章](http://120.78.128.153/rustbook/ch02-00-guessing-game-tutorial.html)）再查看答案。
</details>

<br>
<br>
<details>
    <summary>Step 1 Answer</summary>

```rust, no_run
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

</details>