# minigrep

## Installing

`$ cargo install --path .`

## Running

```
$ minigrep frog poem.txt
How public, like a frog
```

```
$ minigrep body poem.txt
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

```
$ minigrep monomorphization poem.txt
```

```
$ minigrep to poem.txt
Are you nobody, too?
How dreary to be somebody!
```

```
$ IGNORE_CASE=1 minigrep to poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

```
$ echo "hello world" | minigrep hello
hello world
```

```
$ echo "Hello World" | IGNORE_CASE=1 minigrep hello
Hello World
```