
Step 5 

将 parse_config 改写为 Config::new 这个结构体构造函数，即 impl Config { fn new() }

它也是一个结构体关联函数 (函数的参数里面没有 &self)， 关联函数的调用要用双冒号，即 Config::new() 这样的形式调用。  

(另外，结构体函数参数中包含 &self 的是方法函数，使用点号调用)

<br>
<br>
<details>
    <summary>Step5 answer</summary>

```rust, no_run
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

</details>