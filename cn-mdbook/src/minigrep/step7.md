
Step 7

从 main 提取业务处理逻辑

将 main 中读取文件内容提取到一个 run() 函数中 (未来还会在这里面写行搜索和对比逻辑)   


<br>
<br>
<details>
    <summary>查看提示</summary>

将 let contents = fs::read_to_string(config.filename) 语句提取到 run 函数中，使用 config: Config 作为它的参数。

改完运行 cargo run test poem.txt ， 这时应该能够正常运行。

</details>


<br>
<br>
<details>
    <summary>Step 7 answer</summary>

```rust, no_run
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// --snip--
```
</details>

