
Step3 

比较猜测的数字和秘密数字

<br>
<br>
<details>
    <summary>查看提示1</summary>

导入 std::cmp::Ordering, 并使用 match guess.cmp(&secret_number) 来按比较结果输出不同文字。

</details>


<br>
<br>
<details>
    <summary>Step3 Answer 1</summary>

```rust, no_run
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ---snip---

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

</details>


<br>
<br>
<p>这里编译还会有问题</p>

<details>
    <summary>查看提示2</summary>

注意要用 parse() 将 guess 转为数字再与 secret_number 进行比较.
</details>

<br>
<br>
<details>
    <summary>Step3 Answer 2</summary>

```rust, no_run

    // --snip--

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }


```

</details>