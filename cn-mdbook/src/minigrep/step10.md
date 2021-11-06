
The Book 原文的 [12.4. 采用测试驱动开发完善库的功能](http://120.78.128.153/rustbook/ch12-04-testing-the-librarys-functionality.html) 比较长，为了方便练习我们把它分为若干个步骤，每个步骤前进一小步，这是更符合人类大脑的学习方式。  

Step 10

测试驱动开发（Test Driven Development, TDD）的模式，是指先写出期望代码的功能的测试，然后再写出真实能用的代码。这也算是一种目标管理法吧。

测试驱动开发 (TDD) 是一个软件开发技术，它遵循如下步骤：

1. 编写一个失败的测试，并运行它以确保它失败的原因是你所期望的。
2. 编写或修改足够的代码来使新的测试通过。
3. 重构刚刚增加或修改的代码，并确保测试仍然能通过。
4. 从步骤 1 开始重复！

本节先写一个测试结构，主要是测试未来要写的一个 search 函数的功能。


<br>
<br>
<details>
    <summary>查看提示</summary>

练习手打测试代码的结构

未来要完成的函数的签名为: search(query, content) -> Vec<&str> 

</details>


<br>
<br>
<details>
    <summary>Step 10 answer</summary>

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
}
```
</details>
