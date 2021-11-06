Step 12

在 run 函数中使用 search 函数，使整个程序达到可以工作的状态  


<br>
<br>
<details>
    <summary>查看提示</summary>

在 run 函数中运行 search 函数，并将返回的 Vec 结果打出来。

- 先直接用 {:?} 打出来看看
- 改造成用 for 循环，迭代打出每一行 

</details>


<br>
<br>
<details>
    <summary>Step 12 answer 1</summary>

```rust, no_run
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results: Vec<&str> = search(&config.query, &contents);
    println!("{:?}", results);

    Ok(())
}
```

现在运行 cargo run body poem.txt 看看结果
</details>



<br>
<br>
<details>
    <summary>Step 12 answer 2</summary>

```rust, no_run
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results: Vec<&str> = search(&config.query, &contents);

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

现在再运行 cargo run body poem.txt 看看结果

</details>

<br>
<hr>
<br>

到这里，The Book 12.4. 采用测试驱动开发完善库的功能 就已经完成了。  
