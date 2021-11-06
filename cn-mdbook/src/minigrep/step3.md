
The Book 原文的 [12.3. 重构改进模块性和错误处理](http://120.78.128.153/rustbook/ch12-03-improving-error-handling-and-modularity.html) 比较长，为了方便练习我们把它分为若干个步骤[^1]，每个步骤前进一小步，这是更符合人类大脑的学习方式。  

<br>
Step3 

提取参数解析器

将上一步骤提取的 query 和 filename 参数值组合进一个元组(tuple)，并将提取参数值的过程放进一个函数 parse_config

<br>
<br>
<details>
    <summary>查看提示</summary>

使用小括号将 query 和 filename 组合成一个元组, 即 (query, filename), 并将提取参数值的过程放进一个函数 parse_config。

</details>


<br>
<br>
<details>
    <summary>Step3 Answer</summary>

```rust, no_run
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```

</details>

<br>

---

[^1]: The Book 12.3 拆分为本教程的 step 3 到 step 9