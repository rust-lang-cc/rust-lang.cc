
Step 5 验证请求并有选择的进行响应

只是为了学习知识，这里我们对于根页面请求就返回 hello.html, 其它请求一律返回 404 页面。  

在项目根目录 (即 Cargo.toml 所在目录) 创建一个新文件，404.html，内容如下：

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>404 NOT FOUND</title>
    </head>
    <body>
        <h1>Oops!</h1>
        <p>Sorry, I don't know what you're asking for.</p>
    </body>
</html>
```

修改 handle_connection 来对 GET / HTTP/1.1 和其它请求返回不同的页面。


<br>
<br>
<details>
    <summary>查看提示 1</summary>

修改 handle_connection() 函数，如果请求是以 "GET / HTTP/1.1\r\n" 开头就返回 HTTP/1.1 200 OK 状态行及头部及 hello.html 页面。  

浏览页面 [http://127.0.0.1:7878/](http://127.0.0.1:7878/) ，这时前台会看到 Hello 页面。

</details>


<br>
<br>
<details>
    <summary>Step 5 answer 1</summary>

```rust, no_run
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Lenth: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        todo!();  // 请看 小步骤 2
    }
}
```
</details>



<br>
<br>
<details>
    <summary>查看提示 2</summary>

否则，返回 HTTP/1.1 404 NOT FOUND 状态行及头部及 4.4.html 页面。  

浏览页面 [http://127.0.0.1:7878/another_page](http://127.0.0.1:7878/another_page) ，这时前台会看到 404 页面。

</details>


<br>
<br>
<details>
    <summary>Step 5 answer 2</summary>

```rust, no_run
// --snip--

} else {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let contents = fs::read_to_string("404.html").unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```
</details>
