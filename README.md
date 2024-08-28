# wc

## Usage
```
// Run with stdin
➜  wc git:(main) ✗ echo "hello world" | cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/wc`
       1        2       12

// Use file as input
➜  wc git:(main) ✗ cat src/main.rs | cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/wc`
      48      123     1178
```
