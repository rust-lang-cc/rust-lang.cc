
Step 6 

处理无效输入

<br>
<br>
<details>
    <summary>Step6 Answer</summary>

```rust, no_run
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--

```
</details>
