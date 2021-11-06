
Step 9

将 main.rs 中的业务代码拆分到 lib.rs 中。 这里新建 src/lib.rs 实际上是新建了一个库 crate, 现在这个项目中有一个二进制 crate main.rs 和一个库 crate lib.rs ，它们的名称都是 Cargo.toml 里面的 package name 设置的名称。 


<br>
<br>
<details>
    <summary>小步骤 1: 新建 src/lib.rs 并移入相应内容</summary>

将如下代码从 src/main.rs 移动到新文件 src/lib.rs 中：

- run 函数定义
- 相关的 use 语句
- Config 的定义
- Config::new 函数定义

</details>


<br>
<br>
<details>
    <summary>Step 9 小步骤 1 answer</summary>

```rust, no_run
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}
```
</details>



<br>
<hr>
<br>
<details>
    <summary>小步骤 2: 在 main 中导入库 crate 并使用它</summary>

注意这里导入的 minigrep 是这个项目的库名称，请确保 Cargo.toml 里面 package name 设置的是这个名称。

</details>


<br>
<br>
<details>
    <summary>Step 9 小步骤 2 answer</summary>

```rust, no_run
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}
```
</details>

<br>
<hr>
<br>
现在就完成了 The Book 12.3 的内容。