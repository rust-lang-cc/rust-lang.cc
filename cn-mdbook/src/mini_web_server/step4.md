
Step 4 返回真正的 HTML

在项目根目录 (即 Cargo.toml 所在目录) 创建一个新文件，hello.html，内容如下：

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Hello!</title>
    </head>
    <body>
        <h1>Hello!</h1>
        <p>Hi from Rust</p>
    </body>
</html>
```

修改 handle_connection 来读取 HTML 文件，将其加入到响应的 body 中，并发送


<br>
<br>
<details>
    <summary>查看提示</summary>

修改 handle_connection() 函数，读取 HTML 文件，将其加入到响应的 body 中，并发送。  

浏览页面 [http://127.0.0.1:7878/](http://127.0.0.1:7878/) ，这时前台会看到一个基本的 Hello 页面。

</details>


<br>
<br>
<details>
    <summary>Step 4 answer</summary>

```rust, no_run
use std::fs;
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Lenth: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```
</details>
