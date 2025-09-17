# minigrep

## Installing

`$ cargo install --path .`

## Running

```
$ minigrep frog poem.txt
Searching for frog
In file poem.txt
How public, like a frog
```

```
$ minigrep body poem.txt
Searching for body
In file poem.txt
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

```
$ minigrep monomorphization poem.txt
Searching for monomorphization
In file poem.txt
```

```
$ minigrep to poem.txt
Searching for to
In file poem.txt
Are you nobody, too?
How dreary to be somebody!
```

```
$ IGNORE_CASE=1 minigrep to poem.txt
Searching for to
In file poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```