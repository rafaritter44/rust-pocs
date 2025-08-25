# minigrep

## Running

```
cargo run -- frog poem.txt
   Compiling minigrep v0.1.0 (/Users/rafaritter44/git/rust-pocs/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/minigrep frog poem.txt`
Searching for frog
In file poem.txt
How public, like a frog
```

```
cargo run -- body poem.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep body poem.txt`
Searching for body
In file poem.txt
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

```
cargo run -- monomorphization poem.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/minigrep monomorphization poem.txt`
Searching for monomorphization
In file poem.txt
```