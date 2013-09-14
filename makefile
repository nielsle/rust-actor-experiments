# You should really use rustpkg directly instead of this 
# makefile. I just made it to remember the commands

ARCH=x86_64-unknown-linux-gnu

build:
	rm -f .rust/rustpkg_db.json  #Issue 9112
	rustpkg build actor

install: build
	rm -f .rust/rustpkg_db.json  #Issue 9112
	rustpkg install actor

examples: install
	rm -f .rust/rustpkg_db.json  #Issue 9112
	rustpkg build examples/counter
	rustpkg build examples/sieve
	rustpkg build examples/squarer
	rustpkg build examples/pingpong

test: build
	build/$(ARCH)/actor/actortest
	
clean:
	rm -rf build
	rm -rf lib
	rm -rf bin

.PHONY: build test clean


