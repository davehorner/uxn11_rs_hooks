# Makefile for building Rust and C static library together

.PHONY: all clean c-lib rust

all: c-lib rust

c-lib:
	$(MAKE) -C c

rust:
	cargo build

install:
	sudo cp c/libuxn11.a /usr/local/lib/
	sudo cp c/libuxn11.a ./libuxn11.a
	sudo cp c/libuxn11.a ./uxn11.lib
	sudo cp c/libuxn11.a ./libuxn11.lib
	sudo cp c/uxn11.h /usr/local/include/
	sudo ldconfig

uninstall:
	sudo rm -f /usr/local/lib/libuxn11.a
	sudo rm -f /usr/local/include/uxn11.h
	sudo ldconfig

clean:
	$(MAKE) -C c clean
	cargo clean

test-c: test_uxn11

test_uxn11: test_uxn11.c
	gcc test_uxn11.c -o test_uxn11 -L/usr/local/lib -l:libuxn11.a -I/usr/local/include -lX11 -lutil

run-test-c: test_uxn11
	./test_uxn11
