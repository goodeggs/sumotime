sumotime
--------

`sumotime` aims to a be command line tool that will time a command, optionally `SIGKILL` it after some timeout, and send the data to Sumo Logic. It streams the output of the command and passes through the exit code, or `124` if the command ran over the timeout and was killed.

Releasing
===========

```
$ brew install goodeggs/delivery-eng/ghr FiloSottile/musl-cross/musl-cross
$ curl https://sh.rustup.rs -sSf | sh
$ rustup target add x86_64-unknown-linux-musl
$ # update version in Cargo.toml
$ git add Cargo.toml
$ git commit -m 'vX.X.X'
$ git tag -am vX.X.X vX.X.X
$ git push
$ GITHUB_TOKEN="FIXME" ./release.sh
```
