
Step 3 编写响应

了解 Web 响应的格式： 

HTTP-Version Status-Code Reason-Phrase CRLF  
headers CRLF  
CRLF  
message-body  

修改 handle_connection() , 使用 HTTP/1.1 200 OK\r\n\r\n 作为响应。


<br>
<br>
<details>
    <summary>查看提示</summary>

修改 handle_connection() 函数。  

使用 stream.write(response.as_bytes()) 和 stream.flush() 将响应写到客户端。  

浏览页面 [http://127.0.0.1:7878/](http://127.0.0.1:7878/) ，这时前台会是一个空白页面，不再是前面步骤的错误页面了。

</details>


<br>
<br>
<details>
    <summary>Step 3 answer</summary>

```rust, no_run
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```
</details>