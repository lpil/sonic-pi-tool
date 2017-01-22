[![Build Status](https://travis-ci.org/lpil/sonic-pi-tool.svg?branch=master)](https://travis-ci.org/lpil/sonic-pi-tool)

Sonic Pi Tool
=============

`sonic-pi-tool` is a handy command line utility for playing with the Sonic Pi
server. It can be used instead of the Sonic Pi GUI for all your music making
needs :)

It's ideal for use with [sonicpi.vim][sonicpi.vim].
[sonicpi.vim]: https://github.com/dermusikman/sonicpi.vim


## Installation

If you have the [Rust programming language][rust-install] installed Sonic Pi
Tool can be installed like so:

```sh
cargo install https://github.com/lpil/sonic-pi-tool/
```

[rust-install]: https://www.rust-lang.org/en-US/install.html

Other installation methods will (hopefully) come at a later date.


## Usage

- [check](#check)
- [eval](#eval)
- [eval-file](#eval-file)
- [eval-stdin](#eval-stdin)
- [stop](#stop)

### `check`

```sh
sonic-pi-tool check
# => Sonic Pi server listening on port 4557
```

Used to check if the Sonic Pi server is running. If the server isn't running
many of the tool's commands (such as `eval`) will not work.

This command returns a non-zero exit code if the server is not running.


### `eval`

```sh
sonic-pi-tool eval "play :C4"
# *ding*
```

Take a string Sonic Pi code and send it to the Sonic Pi server to be
played.


### `eval-file`

```sh
sonic-pi-tool eval-file path/to/code.rb
# *music*
```

Read Sonic Pi code from a file and send it to the Sonic Pi server to be
played.


### `eval-stdin`

```sh
echo "play :C4" | sonic-pi-tool eval-stdin
# *ding*
```

Read Sonic Pi code from standard in and send it to the Sonic Pi server to be
played.


### `stop`

Stop all jobs running on the Sonic Pi server, stopping the music.

```sh
sonic-pi-tool stop
# *silence*
```


## Other tools

In addition to `sonic-pi-tool` this project contains `sonic-pi-pipe` and
`sonic-pi-logs`. These two programs came first and are written in Go. Once
`sonic-pi-tool` has feature parity with them they will be deprecated.

Read more about these tools [here][old].

[old]: https://github.com/lpil/sonic-pi-tool/tree/master/old
