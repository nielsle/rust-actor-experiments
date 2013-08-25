PKGNAME=actor

build:
	rustpkg build $(PKGNAME)

install:
	rustpkg install $(PKGNAME)

test:
	build/$(PKGNAME)/$(PKGNAME)test
	
clean:
	rustpkg clean $(PKGNAME)
	

.PHONY: build test clean


