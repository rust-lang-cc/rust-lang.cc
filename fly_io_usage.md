// fly_io_usage.md

TL;DR
---
c:\>pwsh 
iwr https://fly.io/install.ps1 -useb | iex
mkdir rustlangcc && cd rustlangcc
touch index.html
flyctl auth login
flyctl init
flyctl deploy
flyctl info
flyctl status
flyctl open
flyctl ips list
flyctl scale show

input content in index.html
login fly.io with github
加信用卡信息，否则不能开 app
v4   213.188.209.113
v6   2a09:8280:1::a:384
http://rustlangcc.fly.dev/
http://rust-lang.cc/
https://rust-lang.cc
@: A @ 213.188.209.113
www: A @ 213.188.209.113
AAAA @ 2a09:8280:1::a:384
www: AAAA @ 2a09:8280:1::a:384

添加 SSL 证书  
flyctl certs create rust-lang.cc 
---
另:

可以对指定的 image 生成 fly.toml 并运行:  
flyctl launch --image flyio/hellofly:latest

当对一个已经存在的本地image生成 fly.toml, 可以运行如下(而不是 flyctl init):  
flyctl apps create

先保存验证信息到 docker, 再 docker pull 拉到本地  
flyctl auth docker
docker image pull registry.fly.io/rustlangcc:deployment-1625727482
---
另：问题

由于前面 flyctl init 的时候选的 static, 导致使用的是 goStatic 这个 docker image, 看它的 Dockerfile, 是从编译一个 go 语言的最简单的静态服务文件，放到 scratch 镜像里面。结果它整个镜像只有一个 /goStatic 可执行程序，其它的 sh bash ls cp等都没有。后面通过 flyctl deploy 的时候，只是把自己的 index.html 放到这个镜像里面去运行。

问题是如果想要从这个镜像里面拿出数据出来，没有办法。虽然能用上面的办法先验证再 docker pull, 但到了本地，还是没有办法导出里面的数据。只能运行起来然后从打开的网页查看源代码，但这并不是可行的办法。  

后续还是要用其它的镜像做底本才行。多看看其它的 flyio 的 examples, 以及 Dockerfile 的应用.  
---

> Install flyctl on Windows
c:\>pwsh  
PS C:\>  
iwr https://fly.io/install.ps1 -useb | iex

PS C:\>  
mkdir rustlangcc && cd rustlangcc
touch index.html

```
<html>
  <head>
    <title>
      Hello from Fly for rust-lang.cc
    </title>
  </head>
  <body>
    <h1>Hello from Fly for rust-lang.cc with a static web site</h1>
  </body>
</html>
```

PS C:\> 
flyctl auth login
// open a browser and login with Github account

flyctl init
App Name (leave blank to use an auto-generated name) rustlangcc
Automatically selected personal organization: jackymaossi@gmail.com
Select builder: (use arrow key on keyboard to choose static )
static
    Web server builtin
We need your payment information to continue! Add a credit card or buy credit: https://fly.io/organizations/personal
add credit card in this webpage.
rerun: flyctl init
final output:
```
Builtins use port 8080
New app created
  Name         = rustlangcc
  Organization = personal
  Version      = 0
  Status       =
  Hostname     = <empty>
             
App will initially deploy to iad (Ashburn, Virginia (US)) region

Wrote config file fly.toml
```
It will make a fly.toml file automatically.

// fly.toml
```
# fly.toml file generated for rustlangcc on 2021-07-07T23:28:29+08:00

app = "rustlangcc"

kill_signal = "SIGINT"
kill_timeout = 5

[build]
  builtin = "static"

[env]

[experimental]
  allowed_public_ports = []
  auto_rollback = true

[[services]]
  http_checks = []
  internal_port = 8080
  protocol = "tcp"
  script_checks = []

  [services.concurrency]
    hard_limit = 25
    soft_limit = 20
    type = "connections"

  [[services.ports]]
    handlers = ["http"]
    port = 80

  [[services.ports]]
    handlers = ["tls", "http"]
    port = 443

  [[services.tcp_checks]]
    grace_period = "1s"
    interval = "15s"
    restart_limit = 6
    timeout = "2s"

```

flyctl deploy
Deploying rustlangcc
==> Validating app configuration
--> Validating app configuration done
Services
TCP 80/443 ⇢ 8080
?[33mWARN ?[0mError connecting to local docker daemon: error during connect: Get "http://192.168.99.100:2376/_ping": dial tcp 192.168.99.100:2376: connectex: A connection attempt failed because the connected party did not properly respond after a period of time, or established connection failed because connected host has failed to respond.
Waiting for remote builder fly-builder-divine-violet-2885... starting ⡿ Creating WireGuard peer "interactive-PC-20190716CCEI-jackymaossi-gmail-com-76"
 in region "iad" for organization personal
Remote builder fly-builder-divine-violet-2885 ready
==> Creating build context
--> Creating build context done
==> Building image with Docker
Step 1/3 : FROM pierrezemb/gostatic
latest: Pulling from pierrezemb/gostatic
275de7d121f6: Pull complete
Digest: sha256:242dd32c2ea85119c80de73e1843b8151f715c7b679649fc81de3ac007d5e904
Status: Downloaded newer image for pierrezemb/gostatic:latest
 ---> 04a29dc9dd54
Step 2/3 : COPY . /srv/http/
 ---> 26eae39521eb
Step 3/3 : CMD ["-port","8080"]
 ---> Running in 6589856922f6
 ---> f51bc3bbf262
Successfully built f51bc3bbf262
Successfully tagged registry.fly.io/rustlangcc:deployment-1625671830
--> Building image done
==> Pushing image to fly
The push refers to repository [registry.fly.io/rustlangcc]
3d5d8846a9b0: Pushed
da87387fa5a0: Pushed
deployment-1625671830: digest: sha256:853e400dafcc604de7812471192e0b6fc99e3cc65397b03525a231b283d0218e size: 735
--> Pushing image done
{sha256:f51bc3bbf2620f779ca96b0a1fb7592267bf7ae6a9cd2203aeadb9c7df51268a [registry.fly.io/rustlangcc:deployment-1625671830] [registry.fly.io/rustlangcc@sha256:853e400dafcc604de7812471192e0b6fc99e3cc65397b03525a231b283d0218e] sha256:26eae39521ebb76f03b426ab423f5fa023c62d67ea17aaeb881e2b7e4485ea5f  2021-07-07T15:31:44.419266681Z 6589856922f6d08ed8fd955f5004d4c473b928fe5d74f9f5c09d15d0ccf04386 0xc000277180 20.10.6  0xc0002772c0 amd64  linux  1860782 1860782 {map[LowerDir:/data/docker/overlay2/a43ad72e4f5415b8c39538a41a332c90c9512318ad71f3a255bf976d5859f934/diff MergedDir:/data/docker/overlay2/285797ca522275aa0f0803f6c8ba693e39641e02172c6b3d6cb87ca7557ea881/merged UpperDir:/data/docker/overlay2/285797ca522275aa0f0803f6c8ba693e39641e02172c6b3d6cb87ca7557ea881/diff WorkDir:/data/docker/overlay2/285797ca522275aa0f0803f6c8ba693e39641e02172c6b3d6cb87ca7557ea881/work] overlay2} {layers [sha256:da87387fa5a0fc0e62108789d20a7a7af3bd33b0e14a06eb81b434b83c444fff sha256:3d5d8846a9b0085cb5b51d339efa6d94575642bfaffe2f15f1b627b8311309ed] } {2021-07-07 15:31:44.423644901 +0000 UTC}}
Image: registry.fly.io/rustlangcc:deployment-1625671830
Image size: 1.9 MB
==> Creating release
Release v0 created

You can detach the terminal anytime without stopping the deployment
Monitoring Deployment

1 desired, 1 placed, 1 healthy, 0 unhealthy [health checks: 1 total, 1 passing]
--> v0 deployed successfully


flyctl info
App
  Name     = rustlangcc
  Owner    = personal
  Version  = 0
  Status   = running
  Hostname = rustlangcc.fly.dev

Services
PROTOCOL PORTS
TCP      80 => 8080 [HTTP]
         443 => 8080 [TLS, HTTP]

IP Adresses
TYPE ADDRESS            CREATED AT
v4   213.188.209.113    10m53s ago
v6   2a09:8280:1::a:384 10m53s ago

flyctl status
App
  Name     = rustlangcc
  Owner    = personal
  Version  = 0
  Status   = running
  Hostname = rustlangcc.fly.dev

Deployment Status
  ID          = f1367fa5-e03d-baa9-a600-a7a42684803d
  Version     = v0
  Status      = successful
  Description = Deployment completed successfully
  Instances   = 1 desired, 1 placed, 1 healthy, 0 unhealthy

Instances
ID       VERSION REGION DESIRED STATUS  HEALTH CHECKS      RESTARTS CREATED
ee04f641 0       iad    run     running 1 total, 1 passing 0        10m45s ago

flyctl open
Opening http://rustlangcc.fly.dev/

flyctl ips list
TYPE ADDRESS            CREATED AT
v4   213.188.209.113    12m37s ago
v6   2a09:8280:1::a:384 12m37s ago

flyctl scale show
VM Resources for rustlangcc
        VM Size: shared-cpu-1x
      VM Memory: 256 MB
          Count: 1
-------------
That's all for now. 
2021-07-07 23:45 Beijing Time.  
Record in China Shenzhen


后续要自定义域名  
https://fly.io/docs/app-guides/custom-domains-with-fly/  
可以使用 X-Forwarded-Host 设定nginx 将客户端的请求代理到新的地址  
直接支持 IPv4 和 IPv6 连接, 两种方式  
- 使用 CNAME
- 设置 A 和 AAAA 记录
CNAME 就像一个指针，如果设置自己的域名 CNAME 指向 nginxproxy.fly.dev ，这种方式最直接，但有几个小问题：
  - 这比第二种方式会慢一点
  - 它限制了你对域名的其它操作，对于顶级域名来说，设置了 CNAME，就不能再增加 MX 记录和其它更多有意义的操作。如果不是用顶级域名的话，设置 CNAME 是一个最快捷的方式  
设置 A 和 AAAA 记录，没有上面 CNAME 的所有顾虑。当通过 flyctl ips list 看到你的 app 的两个 ip (一个ipv4, 一个 ipv6)，直接设置 A 和 AAAA 记录到这个地址就好了。  
添加 SSL 证书  
flyctl certs create rust-lang.cc  
// 如果要添加字域名 wildcard 匹配:  
// flyctl cert create "*.rust-lang.cc"  
flyctl certs show rust-lang.cc
// 上述命令运行后要可以看到 Configured: true, Status: ready, Issued:RSA and/or ECDSA.   
// 如果无误，那么域名在 fly.io 就已经设置好了。
```
You are creating a certificate for rust-lang.cc
We are using lets_encrypt for this certificate.

You can validate your ownership of rust-lang.cc by:

1: Adding an AAAA record to your DNS service which reads:

    AAAA @ 2a09:8280:1::a:384

 OR

1: Adding an CNAME record to your DNS service which reads:

    CNAME _acme-challenge.rust-lang.cc => rust-lang.cc.zjmez.flydns.net.
```

flyctl certs create rust-lang.cc
Error: Hostname already exists on app

flyctl certs show rust-lang.cc
```
The certificate for rust-lang.cc has not been issued yet.

Hostname                  = rust-lang.cc

DNS Provider              = dns

Certificate Authority     = Let's Encrypt

Issued                    =

Added to App              = 2 minutes ago

Source                    = fly

Your certificate for rust-lang.cc is being issued. Status is Awaiting certificates.
```

flyctl certs check rust-lang.cc
```
The certificate for rust-lang.cc has been issued.
Hostname                  = rust-lang.cc

DNS Provider              = dns

Certificate Authority     = Let's Encrypt

Issued                    = ecdsa,rsa

Added to App              = 6 minutes ago

Source                    = fly
```

flyctl certs show rust-lang.cc
```
The certificate for rust-lang.cc has been issued.

Hostname                  = rust-lang.cc

DNS Provider              = dns

Certificate Authority     = Let's Encrypt

Issued                    = ecdsa,rsa

Added to App              = 6 minutes ago

Source                    = fly
```

flyctl certs list
Host Name                 Added                Status
rust-lang.cc              7 minutes ago        Ready

在 https://fly.io/docs/app-guides/custom-domains-with-fly/ 有更多关于通过 GraphQL 来自动获取 SSL 证书的流程。

修改 index.html ，再次部署。  

flyctl deploy
```
Deploying rustlangcc
==> Validating app configuration
--> Validating app configuration done
Services
TCP 80/443 ⇢ 8080
?[33mWARN ?[0mError connecting to local docker daemon: error during connect: Get "http://192.168.99.100:2376/_ping": dial tcp 192.168.99.100:2376: connectex: A connection attempt failed because the connected party did not properly respond after a period of time, or
established connection failed because connected host has failed to respond.
Remote builder fly-builder-quiet-field-8290 ready
==> Creating build context
--> Creating build context done
==> Building image with Docker
Step 1/3 : FROM pierrezemb/gostatic
latest: Pulling from pierrezemb/gostatic
275de7d121f6: Pull complete
Digest: sha256:242dd32c2ea85119c80de73e1843b8151f715c7b679649fc81de3ac007d5e904
Status: Downloaded newer image for pierrezemb/gostatic:latest
 ---> 04a29dc9dd54
Step 2/3 : COPY . /srv/http/
 ---> 7c90def7822f
Step 3/3 : CMD ["-port","8080"]
 ---> Running in 213e562b0dda
 ---> ce26248dcd18
Successfully built ce26248dcd18
Successfully tagged registry.fly.io/rustlangcc:deployment-1625701886
--> Building image done
==> Pushing image to fly
The push refers to repository [registry.fly.io/rustlangcc]
06a6a9614584: Pushed
da87387fa5a0: Layer already exists
deployment-1625701886: digest: sha256:c701d0be7ba9d371d31584de5fec4d1a4fc32ba84a59299db2ac2dc5e412164b size: 736
--> Pushing image done
{sha256:ce26248dcd189ca978b0f21f419d8e8086cab07fffe8434b7f7f40b550d7495b [registry.fly.io/rustlangcc:deployment-1625701886] [registry.fly.io/rustlangcc@sha256:c701d0be7ba9d371d31584de5fec4d1a4fc32ba84a59299db2ac2dc5e412164b] sha256:7c90def7822f78b7b482fd0794246fc09e520419aaa52c837e16cee651df630e  2021-07-07T23:52:22.871252261Z 213e562b0ddadcacd89c27bb636e0f00803f54747f43e88f2b4b05453166da84 0xc000543a40 20.10.6  0xc000543b80 amd64  linux  1870200 1870200 {map[LowerDir:/data/docker/overlay2/a32a765f0e92c80a5efc0e392ff3523f94d573c5bf630657e807189cbb1ca72f/diff MergedDir:/data/docker/overlay2/1fca54953c39597240cd3fdcd58a876a1c541d5e583b0fd865c96208fc158c45/merged UpperDir:/data/docker/overlay2/1fca54953c39597240cd3fdcd58a876a1c541d5e583b0fd865c96208fc158c45/diff WorkDir:/data/docker/overlay2/1fca54953c39597240cd3fdcd58a876a1c541d5e583b0fd865c96208fc158c45/work] overlay2} {layers [sha256:da87387fa5a0fc0e62108789d20a7a7af3bd33b0e14a06eb81b434b83c444fff sha256:06a6a9614584fbfda946f8e6a0fe5e0212fb8c7ffb355852bbb1bc33191e14f4] } {2021-07-07 23:52:22.87541506 +0000 UTC}}
Image: registry.fly.io/rustlangcc:deployment-1625701886
Image size: 1.9 MB
==> Creating release
Release v1 created

You can detach the terminal anytime without stopping the deployment
Monitoring Deployment

1 desired, 1 placed, 1 healthy, 0 unhealthy [health checks: 1 total, 1 passing]
--> v1 deployed successfully
```

看来远程那个 自动生成的 app 是一个 image, 以后不要删除了，只要它不运行，就不会占用过多资源。


```

flyctl ssh establish
Automatically selected personal organization: jackymaossi@gmail.com
Establishing SSH CA cert for organization personal
New organization root certificate:
ssh-ed25519-cert-v01@openssh.com AAAAIHNzaC1lZDI1NTE5LWNlcnQtdjAxQG9wZW5zc2guY29tAAAAIOwHVDhw1IxDEsGWy99CvIbdzrzpgMUK6imLEuzCwntfAAAAIJOezn+0m5dcLG9dbYAZTHlNa0bdRASt+X3YE4qWjAF+AAAAAAAAAAAAAAABAAAAEmZseTpvcmc6MjE1Mjk6cm9vdAAAAAAAAAAAAAAAAP//////////AAAAAAAAAAAAAAAAAAAAMwAAAAtzc2gtZWQyNTUxOQAAACCTns5/tJuXXCxvXW2AGUx5TWtG3UQErfl92BOKlowBfgAAAFMAAAALc3NoLWVkMjU1MTkAAABArwQHrqxswSgAGvKznfytSSX/gHbzSxcDnYNEMCSeneMmm3rmN+affgWiuHEKD4DVjKpQgaiXVkbdwuPhXJiVAw==

```

flyctl ssh console
flyctl ssh console
Creating WireGuard peer "interactive-Jacky1-jackymaossi-gmail-com-208" in region "iad" for organization personal
Connecting to rustlangcc.internal...?? ?[31mError?[0m connect to SSH server: dial: lookup rustlangcc.internal. on fdaa:0:2f20::3: read udp [fdaa:0:2f20:a7b:ce2:0:a:100]:20879: i/o timeout

```

flyctl config env
Environment variables
NAME VALUE

flyctl secrets list
NAME DIGEST DATE

flyctl platform status
Opening https://status.fly.io/

flyctl platform regions

flyctl platform vm-sizes
NAME             CPU CORES MEMORY
shared-cpu-1x    1         256 MB
dedicated-cpu-1x 1         2 GB
dedicated-cpu-2x 2         4 GB
dedicated-cpu-4x 4         8 GB
dedicated-cpu-8x 8         16 GB
```

flyctl list apps
NAME                         STATUS  ORG      DEPLOYED
fly-builder-quiet-field-8290 dead    personal 4 hours ago
rustlangcc                   running personal 4 hours ago

```

flyctl deploy
Deploying rustlangcc
==> Validating app configuration
--> Validating app configuration done
Services
TCP 80/443 ⇢ 8080
?[33mWARN ?[0mError connecting to local docker daemon: Get "http://192.168.99.100:2376/_ping": net/http: HTTP/1.x transport connection broken: malformed HTTP response "\x15\x03\x01\x00\x02\x02".
* Are you trying to connect to a TLS-enabled daemon without TLS?
Remote builder fly-builder-quiet-field-8290 ready
==> Creating build context
--> Creating build context done
==> Building image with Docker
Step 1/3 : FROM pierrezemb/gostatic
 ---> 04a29dc9dd54
Step 2/3 : COPY . /srv/http/
 ---> a185cc186b06
Step 3/3 : CMD ["-port","8080"]
 ---> Running in f5c101be025d
 ---> 0440dc40be47
Successfully built 0440dc40be47
Successfully tagged registry.fly.io/rustlangcc:deployment-1625727482
--> Building image done
==> Pushing image to fly
The push refers to repository [registry.fly.io/rustlangcc]
bd2341e7b1ad: Pushing [=>                                                 ]     512B/1bd2341e7b1ad: Pushed
da87387fa5a0: Layer already exists
deployment-1625727482: digest: sha256:885c3b9b1d447c1299db45090da936af298787a641b274ca83aee87e08844bad size: 736
--> Pushing image done
{sha256:0440dc40be47a720d6560aea28b48fd602cb72cc55b4e89ef9acde2e822b6992 [registry.fly.io/rustlangcc:deployment-1625727482] [registry.fly.io/rustlangcc@sha256:885c3b9b1d447c1299db45090da936af298787a641b274ca83aee87e08844bad] sha256:a185cc186b06d054b57120d97e4ef946af7750c37470067c4f3a78a89457522a  2021-07-08T06:58:35.474439821Z f5c101be025d24c4892b4f7ebd3dee59c0d9e6e371757c2566637e53fa556e23 0xc0004af180 20.10.6  0xc0004af2c0 amd64  linux  1878967 1878967 {map[LowerDir:/data/docker/overlay2/a32a765f0e92c80a5efc0e392ff3523f94d573c5bf630657e807189cbb1ca72f/diff MergedDir:/data/docker/overlay2/37f28033182a4e9d441cea6f0cb63e5ec27c8b8641f6564214fd2af0ef58e97a/merged UpperDir:/data/docker/overlay2/37f28033182a4e9d441cea6f0cb63e5ec27c8b8641f6564214fd2af0ef58e97a/diff WorkDir:/data/docker/overlay2/37f28033182a4e9d441cea6f0cb63e5ec27c8b8641f6564214fd2af0ef58e97a/work] overlay2} {layers [sha256:da87387fa5a0fc0e62108789d20a7a7af3bd33b0e14a06eb81b434b83c444fff sha256:bd2341e7b1ad06fcfeb906d9983c5aa12ac2398dc5aaefbc244d023a29bc9d04] } {2021-07-08 06:58:35.47883104 +0000 UTC}}
Image: registry.fly.io/rustlangcc:deployment-1625727482
Image size: 1.9 MB
==> Creating release
Release v4 created

You can detach the terminal anytime without stopping the deployment
Monitoring Deployment

1 desired, 1 placed, 1 healthy, 0 unhealthy [health checks: 1 total, 1 passing]
--> v4 deployed successfully

------
flyctl auth docker
Authentication successful. You can now tag and push images to registry.fly.io/{your-app}

docker image pull registry.fly.io/rustlangcc:deployment-1625727482
deployment-1625727482: Pulling from rustlangcc
275de7d121f6: Pull complete
11a2a1a7c2ac: Pull complete
Digest: sha256:885c3b9b1d447c1299db45090da936af298787a641b274ca83aee87e08844bad
Status: Downloaded newer image for registry.fly.io/rustlangcc:deployment-1625727482

ref: https://github.com/superfly/flyctl/issues/142

------
