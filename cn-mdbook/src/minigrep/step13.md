Step 13

处理环境变量，使程序能处理大小写敏感的搜索 (如果设置了 CASE_INSENSITIVE，就按大小写不敏感来搜索)

这是 [The Book 12.5. 处理环境变量](http://120.78.128.153/rustbook/ch12-05-working-with-environment-variables.html) 的内容，我们也把它拆分为几个步骤来实现，一步一个脚印

首先，增加两个测试，一个是大小写敏感的，一个是大小写不敏感的。并需要增加一个 search_case_insensitive 函数雏形使程序能 cargo test


<br>
<br>
<details>
    <summary>查看提示</summary>

两个测试函数名分别为 fn case_sensitive() 和 fn case_insensitive()， 对应运行的函数为 search(query, contents) 和 search_case_insensitive(query, contents)

</details>


<br>
<br>
<details>
    <summary>Step 13 answer</summary>

```rust, no_run
#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";
		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}


    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

运行 cargo test，现在还没写 search_caseinsensitive，编译不了
</details>


<br>
<br>
<details>
    <summary>Step 13 answer 2</summary>

写一个 search_case_insensitive 函数 雏形， 使程序能 cargo test

```rust, no_run
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

忽略变量未使用的警告。

可以看到有两个测试通过了 (one_result 和 case_sensitive)， 一个测试没通过 (case_insensitive)

</details>
