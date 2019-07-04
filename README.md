# Playing around with Gotham

I dunno
```
cargo run
# or
docker build -t gotham-rust . && docker run -p 80:80 -it --rm --name gotham gotham-rust

http localhost
# or
wrk -t12 -c400 -d60s http://127.0.0.1
```