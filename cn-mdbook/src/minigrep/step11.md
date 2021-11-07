Step 11

编写 search 函数的雏形并逐步完成它。

分成几个小步骤来写  


<br>
<br>
<details>
    <summary>小步骤 1：写出 search 函数的雏形，并加上生命周期</summary>

先写上不带生命周期的函数，再加上生命周期标注

</details>


<br>
<br>
<details>
    <summary>小步骤 1 search 函数的雏形 answer</summary>

```rust, no_run
pub fn search(query: &str, contents: &str) -> Vec<&str> {
    vec![]
}
```
</details>

<br>
<br>
<details>
    <summary>小步骤 1 加上生命周期标注 answer</summary>

```rust, no_run
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```
</details>



<br>
<hr>
<br>
<details>
    <summary>小步骤 2：使用 lines 方法遍历并对比每一行</summary>

先用 for 循环遍历 contents.lines()，再对比每一行 line 与 query， 是否 line.contains(query)

</details>

<br>
<br>
<details>
    <summary>小步骤 2 用 for 循环遍历 contents.lines() answer</summary>

```rust, no_run
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```
</details>

<br>
<details>
    <summary>小步骤 2 使用 query 去比对每一行 answer</summary>

```rust, no_run
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
        }
    }
}
```
</details>



<br>
<hr>
<br>
<details>
    <summary>小步骤 3：存储匹配的行</summary>

新建可变变量 results = Vec::new()

在循环中将每一个匹配的行存入其中

并让函数返回这个 results

</details>

<br>
<br>
<details>
    <summary>小步骤 3 answer</summary>

```rust, no_run
pub fn search(query: &str, contents: &str) -> Vec<&str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```
</details>

<br>
<hr>
<br>

现在可以运行 cargo test ，测试能通过！

了不起的工作，给自己鼓掌！
