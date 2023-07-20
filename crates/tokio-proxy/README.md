# tokio-proxy

This simple lib helps to do tokio tcp connect via proxy (currently only socks5 is supported).

The `connect` function is similar to `TcpStream::connect`, with an extra argument to specify proxy url string.

```rust
pub async fn connect<A: ToSocketAddrsExt, T: AsRef<str>>(
    addr: A,
    proxy_url: T,
) -> io::Result<TcpStream>
```

## example

You could check in examples/env_proxy.rs.

It checks env var `all_proxy` as proxy url, if non-exist, connect directly to httpbin.org.

```rust
    rt.block_on(async move {
        let mut s = match env::var("all_proxy") {
            Ok(proxy) => tokio_proxy::connect("httpbin.org:80", proxy).await.unwrap(),
            Err(_) => TcpStream::connect("httpbin.org:80").await.unwrap(),
        };

        s.write_all(
            b"GET /get HTTP/1.1\r\n\
        Host: httpbin.org\r\n\r\n",
        )
        .await
        .unwrap();

        let mut buf = [0; 512];
        s.read(&mut buf).await.unwrap();

        for line in str::from_utf8(&buf).unwrap().lines() {
            println!("{}", line);
        }
    });
```

If the proxy url uses `socks5` scheme, then it would resolve the domain name locally
and iterate all resolved ip addresses towards proxy, until one succeed or all failed.

If the proxy url uses `socks5h` scheme, then it would send the domain name directly to proxy.

It's just like what curl does.

```
$ all_proxy=socks5h://localhost:8080 cargo run --example env_proxy
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target\debug\examples\env_proxy.exe`
HTTP/1.1 200 OK
Access-Control-Allow-Credentials: true
Access-Control-Allow-Origin: *
...
```
