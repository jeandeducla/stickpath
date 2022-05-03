# stickpath

## run

```
cargo run
```

then you need to input a `width` first and then a `height` (`width` cannot be lower than 3 and `height` cannot be greater than 100, otherwise it will panic).
You then need to enter a valid stickpath in the next `height` lines (see below for a valid example of a stickpath, otherwise it will panic).
The program will return all the possible paths of the corresponding stickpath.


Here is a valid stickpath:
```
A  B  C  # note that there are two spaces between the leters: ['A', ' ', ' ', 'B', ' ', ' ', 'C']
|  |  |  # same here: there are two spaces between the vertical bars
|--|  |
|  |--|
|  |--|
|  |  |
1  2  3
```

## test

```
cargo test
```
