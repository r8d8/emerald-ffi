GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: clean build
  ./examples/main

build:
  $(CARGO_BIN) build --release
  $(GCC_BIN) -o ./examples/main ./examples/main.c -Isrc  -L. -l:target/release/emerald_ffi.so

clean:
  $(CARGO_BIN) clean
  rm -f ./examples/main
