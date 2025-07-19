.PHONY: all clean
.DEFAULT_GOAL := all

CC=cc
EXEC=main
RUST_LIB=rustlib/target/release/libgreetlib.dylib

${RUST_LIB}:
	cargo build --release --manifest-path=rustlib/Cargo.toml

all: ${RUST_LIB}
	${CC} -std=c99 -Wall -Werror -o ${EXEC} main.c -L${dir ${RUST_LIB}} -lgreetlib

clean:
	cargo clean --manifest-path=rustlib/Cargo.toml
	rm -f ${EXEC}
