
Step 2 

目标：生成一个秘密数字

<br>
<details>
    <summary>查看提示</summary>

需要导入 rand::Rng, 并使用 rand::thread_rng().gen_range(1, 101) 生成秘密数字，并打印出来。  

</details>


<br>
<br>
<details>
    <summary>Step2 answer</summary>

```
# Cargo.toml

[dependencies]
rand = "0.5.5"

```

```rust, no_run
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```
</details>