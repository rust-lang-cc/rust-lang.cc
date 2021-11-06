
Step6

修复错误处理

这一节稍有一点复杂，但也非常重要，对以后写程序或看别人的程序都非常有用，主要就是对 Result<Config, Err> 这样的结构的理解。  

这一节分为三个小步骤。


<br>
<br>
<details open>
    <summary>小步骤 1: 改进的 panic</summary>

现在如果运行 cargo run ，后面不带参数，会出现 panic 错误，thread 'main' panicked at 'index out of bounds: ...', 这是一个很笼统的错误信息，修改产生一个手动的 panic 来提供更有意义的错误信息提示。 

</details>


<br>
<br>
<details>
    <summary>小步骤 1 answer</summary>

```rust, no_run
// --snip--
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // --snip--
```
</details>


<br>
<hr>
<br>
<details open>
    <summary>小步骤 2: 从 new 中返回 Result 而不是调用 panic!</summary>

让 new 函数返回 Result<Config, &'static str> 而不是 Config, 将 panic 改写为 return Err()  

<small> ( 现在如果运行 cargo run ，会出现程序错误，因为还没有在 main() 里面处理返回的错误， 这将在小步骤3里面处理。)  </small>  

</details>


<br>
<br>
<details>
    <summary>小步骤 2 answer</summary>

```rust, no_run
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```
</details>



<br>
<hr>
<br>
<details open>
    <summary>小步骤 3: 在 main() 中处理 Config::new 返回的错误</summary>

使用 unwrap_or_else(|err| {}) 形式来处理返回的错误信息  

导入 std::process , 当 Config::new 返回错误的时候说明参数解析出现错误，程序没必要再运行下去，使用 process.exit(1) 退出程序。 

现在运行 cargo run ，看看程序错误提示

</details>


<br>
<br>
<details>
    <summary>小步骤 3 answer</summary>

```rust, no_run
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
```
</details>