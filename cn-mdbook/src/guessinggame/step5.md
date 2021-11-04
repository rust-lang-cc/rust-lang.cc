
Step5 

猜测正确后退出


<br>
<br>
<details>
    <summary>Step5 answer</summary>

```rust, no_run
// --snip--

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
