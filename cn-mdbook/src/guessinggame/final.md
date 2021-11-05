
恭喜你完成猜数字游戏 

<br>
<br>
<details open>
    <summary>完整代码</summary>

```rust, no_run
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

```
</details>


<br>
<br>
<details>
    <summary>效果: </summary> 

在 console 输入 cargo run 编译并开始运行游戏。  

<iframe frameborder="0" width="100%" height="600px" src="https://replit.com/@jackymao/guessinggame?lite=true"></iframe>

</details>