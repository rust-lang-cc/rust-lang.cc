Step 15

现在测试通过了，是时候来写满足我们需求的真实代码了。

分为三个小步骤： 
- 更新 Struct Config { } 定义，增加 case_sensitive 项
- 更新 Config::new 构造函数，增加 case_sensitive 部分
- 更新 run 函数，根据 CASE_INSENSITIVE 环境变量的设置与否，来运行不同的大小写敏感搜索函数


<br>
<br>
<details open>
    <summary>小步骤 1: 更新 Struct Config { } 定义</summary>

增加 case_insensitive 项

</details>


<br>
<br>
<details>
    <summary>小步骤 1 answer</summary>

```rust, no_run
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

</details>



<br>
<hr>
<br>
<details open>
    <summary>小步骤 2: 更新 Config::new 构造函数</summary>

导入 std::env, 并增加 Config::new 的 case_insensitive 部分

</details>


<br>
<br>
<details>
    <summary>小步骤 2 answer</summary>

```rust, no_run
use std::env;

// --snip--

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

</details>



<br>
<hr>
<br>
<details open>
    <summary>小步骤 3: 更新 run 函数</summary>

根据 CASE_INSENSITIVE 环境变量的设置与否，来运行不同的大小写敏感搜索函数

</details>


<br>
<br>
<details>
    <summary>小步骤 3 answer</summary>

```rust, no_run
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

</details>

<br>
<hr>
<br>
现在，分别运行如下命令，看看程序是否如期望的工作。

cargo run to poem.txt

以及  

(Window powershell)
> pwsh
> $env:CASE_INSENSITIVE=1; cargo run to poem.txt
> exit

(Linux 下)
> CASE_INSENSITIVE=1 cargo run to poem.txt
