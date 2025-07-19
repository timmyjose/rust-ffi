.PHONY: clean all
.DEFAULT_GOAL := all

CC=cc
EXEC=main
RUST_LIB=rustlib/target/release/libmathlib.a

${RUST_LIB}:
	cargo build --release --manifest-path=rustlib/Cargo.toml

all: ${RUST_LIB}
	${CC} -std=c99 -Wall -Werror main.c ${RUST_LIB} -o ${EXEC}

clean:
	cargo clean --manifest-path rustlib/Cargo.toml
	rm -f ${EXEC}
