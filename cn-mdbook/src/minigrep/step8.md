
Step 8

将 run 函数的错误返回出来, 并在 main 中进行处理


<br>
<br>
<details>
    <summary>小步骤 1: 从 run 函数中返回错误</summary>

将 run 函数中的 .expect() 改为 ? 符号，并将 Err 放到它的返回结构中，使用 Result<(), Box&lt;dyn Error>> 作为它的返回值。<!-- 这里将 Box<dyn Error> 的第一个 < 符号改为 &lt; 以免最终解析的 html 中出现错误 -->

改完运行 cargo run test poem.txt ， 这时能够正常运行，但会有警告，因为返回的 Error 还没有处理，这将在小步骤 2 里面处理。

</details>


<br>
<br>
<details>
    <summary>Step 8 小步骤 1 answer</summary>

```rust, no_run
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```
</details>



<br>
<hr>
<br>
<details>
    <summary>小步骤 2: 在 main 中处理从 run 函数中返回的错误</summary>

将 run(config) 改为 if let Err(e) = run(config) { }，使用 if let 来检查 run 是否返回一个 Err 值，并在后面的 { } 语句块里面处理这个错误（打印错误并退出程序）。  

<small>这里并不需要处理 Result<(), Box&lt;dyn Error>> 的 Result 的第一个值，因为它是 ()，不需要处理。这是与前面的 Result<config, Err> 有区别的地方。</small> <!-- 这里将 Box<dyn Error> 的第一个 < 符号改为 &lt; 以免最终解析的 html 中出现错误 -->

改完运行 cargo run 和 cargo run test poem.txt ， 确保能够正常运行。

</details>


<br>
<br>
<details>
    <summary>Step 8 小步骤 2 answer</summary>

```rust, no_run
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```
</details>

