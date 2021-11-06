Step 14

实现 search_case_insensitive 函数


<br>
<br>
<details>
    <summary>查看提示</summary>

类似于 search 函数， 但在合适的地方添加 .to_lowercase()， 使程序实现大小写不敏感的搜索

</details>


<br>
<br>
<details>
    <summary>Step 14 answer</summary>

```rust, no_run

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

现在运行 cargo test，所有测试都能通过
</details>