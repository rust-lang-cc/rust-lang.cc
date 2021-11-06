# Step16

将错误信息输出到标准错误而不是标准输出 [^1]



<br>
<br>
<details>
    <summary>查看提示</summary>

标准库提供了 eprintln! 宏来打印到标准错误流，所以将 main.rs 中两个调用 println! 打印错误信息的位置替换为 eprintln! 即可

</details>


<br>
<br>
<details>
    <summary>Step 16 answer</summary>

```rust, no_run
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
```

</details>


<br>
<hr>
<br>
运行以下命令测试一下：  

cargo run > output.txt  
Problem parsing arguments: not enough arguments  

现在我们看到了屏幕上的错误信息，同时 output.txt 里什么也没有，这正是命令行程序所期望的行为。


cargo run to poem.txt > output.txt  

我们并不会在终端看到任何输出，同时 output.txt 将会包含其结果：

Are you nobody, too?  
How dreary to be somebody!  

这一部分展示了现在我们适当的使用了成功时产生的标准输出和错误时产生的标准错误。


---

[^1]: 为什么要 将错误信息输出到标准错误而不是标准输出？ 查看 [The Book 12.6. 将错误信息输出到标准错误而不是标准输出](http://120.78.128.153/rustbook/ch12-06-writing-to-stderr-instead-of-stdout.html)，解释得非常好。