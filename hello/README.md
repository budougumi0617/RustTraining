# 2.4 Process argements

```bash
$ cargo run
   Compiling hello v0.1.0 (file:///Users/shimizu-yoichiro/go/src/github.com/budougumi0617/RustTraining/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.29s
     Running `target/debug/hello`
Usage: gcd NUMBER ...
$ cargo run 42 56
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/hello 42 56`
The greatest comon divisor of [42, 56] is 14
$ cargo run 799459 28823 27347
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/hello 799459 28823 27347`
The greatest comon divisor of [799459, 28823, 27347] is 41
```
