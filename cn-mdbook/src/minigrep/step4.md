
Step 4

使用 Config 结构体来替代 (query, filename) 元组

注意所有使用 query 和 filename 值的地方都要用 config.query 和 config.filename 代替, 并用 clone() 暂时解决 Config 结构体中的所有权冲突。

<br>
<br>
<details>
    <summary>查看提示</summary>

建立 Config 结构体如下：

```rust, no_run
fn main() {
    let args: Vec<String> = env::args().collect();

    // 后续这里的 : Config 类型注释会省略掉，自己心里要明白这里是省略了类型  
    let config: Config = parse_config(&args);  

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

    // --snip--
```

</details>

<br>
<br>
<details>
    <summary>Step4 Answer</summary>

```rust, no_run
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

</details>