#!/bin/zsh

# It is anticpated that this will need to be altered once the project has dependencies:
# https://github.com/dlecan/rust-crosscompiler-arm#optimized-usage

#  --user $(id -u) \
docker run -it --rm \
  -v $(pwd):/source \
  -v ~/.cargo/git:/root/.cargo/git \
  -v ~/.cargo/registry:/root/.cargo/registry \
  dlecan/rust-crosscompiler-arm:stable

# unfortunately, this means that any new .cargo files are owned by root now

# todo - setup ssh keys to deploy, as this requires a password
scp $(pwd)/target/arm-unknown-linux-gnueabihf/release/wildlife_cam pi@192.168.1.100:
