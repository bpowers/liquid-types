# quiet output, but allow us to look at what commands are being
# executed by passing 'V=1' to make, without requiring temporarily
# editing the Makefile.
ifneq ($V, 1)
MAKEFLAGS += -s
endif

# only used in .docker_image_name target
PROJECT    = $(shell basename `git rev-parse --show-toplevel`)
GIT_REV    = $(shell git rev-parse --short HEAD)
IMAGE_NAME = $(PROJECT):$(GIT_REV)

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

.docker_image_name: Makefile Dockerfile
	docker build -t $(IMAGE_NAME) -f ./Dockerfile
	echo $(IMAGE_NAME) >.docker_image_name

docker: .docker_image_name
	docker run --rm -it $(shell cat .docker_image_name)

clean:
	cargo clean
	find . -name '*~'   -type f -print0 | xargs -0 rm -f
	find . -name '*.bk' -type f -print0 | xargs -0 rm -f

.PHONY: all build docker test clean
