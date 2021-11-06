# minigrep 

step1

编写本程序需要使用本地编译或使用 [repl.it 运行空间](../guessinggame/replit.html)

新建一个项目，名称为 minigrep

<br>

目标：读取命令行参数值并打印出来

<br>
<details>
    <summary>简单收集参数并打印</summary>

导入 std::env ，使用 env::args() 获取命令行参数，并使用 collect() 收集到一个数组中，然后打印出来。

使用 cargo run 和 cargo run test poem.txt 分别测试一下

下面有答案。请尽量自己写出来（或复习 [The book 第十二章](http://120.78.128.153/rustbook/ch12-00-an-io-project.html)）再查看答案。
</details>

<br>
<br>
<details>
    <summary>Step 1 Answer 1</summary>

```rust, no_run
use std::env;

fn main() {
    let args: Vec[String] = env::args().collect();
    println!("{:?}", args);
}
```

</details>


<br>
<br>
<details>
    <summary>将参数值保存进变量，并分别打印</summary>

分别将 搜索词 和 文件名 参数保存到对应的参数。

使用 cargo run tet poem.txt 测试一下

</details>

<br>
<br>
<details>
    <summary>Step 1 Answer 2</summary>

```rust, no_run
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

</details>
