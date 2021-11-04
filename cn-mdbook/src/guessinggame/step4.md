
Step 4

使用循环来允许多次猜测

使用 loop

<br>
<br>
<details>
    <summary>Step4 Answer</summary>

```rust, no_run
// --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

</details>