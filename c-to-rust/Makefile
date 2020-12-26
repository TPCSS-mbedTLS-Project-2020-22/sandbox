
all: run


build:
	cargo build
	gcc -o test_c2rust src/test-main.c  target/debug/libtest_c2rust.so


run: build
	./test_c2rust

