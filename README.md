# pshort: a path shortener
I use this as part of my shell prompt as a compromise between only printing the
name of the current directory and displaying the whole path.

## Examples
```sh
$ pshort /usr/lib/systemd/user
/u/l/systemd/user
```
Specify the length at which truncation begins. The name of the current
directory is never truncated
```sh
$ pshort /usr/lib/systemd/user 1
/u/l/s/user
```
`pshort` does not attempt to resolve paths other than those with '~'
```sh
$ cargo run -- .
.
```
There is special handling for hidden directories — those starting with '.'
```sh
$ pshort ~/.config/nvim 1
~/.c/nvim
```

## Building
```sh
$ cargo build --release
```

## Installation
[deploy.sh](./scripts/deploy.sh) builds and copies `pshort` to
`$HOME/.local/bin`, but copying it to anywhere else in your `PATH` works just
as well.
