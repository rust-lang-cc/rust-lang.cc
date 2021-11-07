# mini_web_server

Step1

编写本程序需要使用本地编译，请确保已经按 The Book 第一章在自己的电脑上安装了 Rust 编译环境[^1]

新建一个项目，名称为 mini_web_server

<br>

目标：在 7878 端口接收 TCP 连接，并在后台打印 "Connection established!"

当 cargo run 运行起来以后，用浏览器访问 [http://127.0.0.1:7878/](http://127.0.0.1:7878/)， 可以看到在后台打印出几行 Connection established. 的信息，说明本步骤成功了。

<br>
<details>
    <summary>使用 TcpListener 绑定端口，并在收到传入的流时打印信息</summary>

导入 std::net::TcpListener ，使用 TcpListener::bind 绑定端口，并在 listener.incoming() 收到流时打印信息。


下面有答案。请尽量自己写出来（或复习 [The book 第二十章](http://120.78.128.153/rustbook/ch20-00-final-project-a-web-server.html)）再查看答案。
</details>

<br>
<br>
<details>
    <summary>Step 1 Answer</summary>

```rust, no_run
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        println!("Connection established!");
    }
}
```

</details>

<br>

---

[^1]: 后续有空再来测试一下 gitpod 或 coding (或 fly.io 或 railway.app)
