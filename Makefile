# quiet output, but allow us to look at what commands are being
# executed by passing 'V=1' to make, without requiring temporarily
# editing the Makefile.
ifneq ($V, 1)
MAKEFLAGS += -s
endif

# GNU make, you are the worst.
.SUFFIXES:
%: %,v
%: RCS/%,v
%: RCS/%
%: s.%
%: SCCS/s.%
.SUFFIXES: .rs

all: build

build: test
	cargo build

test:
	cargo test

.docker_image_name: Makefile Dockerfile build.sh
	./build.sh

docker: .docker_image_name
	docker run -it $(shell cat .docker_image_name)

clean:
	cargo clean
	find . -name '*~'   -type f -print0 | xargs -0 rm -f
	find . -name '*.bk' -type f -print0 | xargs -0 rm -f

.PHONY: all build docker test clean
