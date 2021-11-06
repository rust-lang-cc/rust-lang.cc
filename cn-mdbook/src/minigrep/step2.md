
Step 2 

目标：读取一个文本文件并打印出它的内容

将下面的内容保存到项目的根目录（和 Cargo.toml 同一个目录），文件名 poem.txt
```txt
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

<br>
<br>
<details>
    <summary>查看提示</summary>

导入 std::fs, 并使用 fs::read_to_string(filename) 读取文件内容，并打印出来。  

运行 cargo run the poem.txt 测试一下，要能正确打印出文件的内容。

</details>


<br>
<br>
<details>
    <summary>Step2 answer</summary>

```rust, no_run
use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```
</details>