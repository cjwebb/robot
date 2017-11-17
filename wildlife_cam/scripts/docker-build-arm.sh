#!/bin/zsh

# It is anticpated that this will need to be altered once the project has dependencies:
# https://github.com/dlecan/rust-crosscompiler-arm#optimized-usage

docker run -it --rm \
  -v $(pwd):/source \
  dlecan/rust-crosscompiler-arm:stable
