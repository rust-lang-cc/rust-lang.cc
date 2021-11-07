
Step 6 少量代码重构

目前 if 和 else 块中的代码有很多的重复, 基于 DRY 原则 (Don't repeat yourself), 我们重构一下。

重构 handle_connection 函数  



<br>
<br>
<details>
    <summary>查看提示</summary>

重构 handle_connection 函数，将 if else 里面返回的状态行和文件名提取成两个语句，其它的相同语句只用写一遍就好了，完成重构。（可重用的语句有 let contents, let response, stream.write, stream.flush）

浏览页面 [http://127.0.0.1:7878/](http://127.0.0.1:7878/) 和 [http://127.0.0.1:7878/another_page](http://127.0.0.1:7878/another_page) ，确保看到正确的 Hello 和 404 页面。

</details>


<br>
<br>
<details>
    <summary>Step 6 answer</summary>

```rust, no_run
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents); // 对比前面的代码发现这里省略了 contents.len()

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```
</details>



<br>
<br>
<hr>

到这里 [The Book 20.1. 单线程 web server](http://120.78.128.153/rustbook/ch20-01-single-threaded.html) 就完成了。  