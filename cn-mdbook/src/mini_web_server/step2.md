
Step 2 读取请求

了解 HTTP 请求的格式:  

Method Request-URI HTTP-Version CRLF  
headers CRLF  
CRLF  
message-body  

<br>
使用 stream.read() 读取请求，并打印出来 


<br>
<br>
<details>
    <summary>查看提示</summary>

use std::io::prelude::*;  
use std::net::TcpStream;  

在 main 函数中调用新的 handle_connection 函数并向其传递 stream 来处理流。

新建 handle_connection() 函数。  

使用 stream.read(&mut buffer) 方式将流的请求读取到 buffer 里面，并使用 String::from_utf8_lossy(&buffer[..]) 方式取出并打印出来。  

浏览页面 [http://127.0.0.1:7878/](http://127.0.0.1:7878/) ，查看后台打印出的请求详情。

</details>


<br>
<br>
<details>
    <summary>Step 2 answer</summary>

```rust, no_run
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```
</details>