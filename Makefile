SHELL := /usr/bin/env bash

all:

project.init:
	cargo generate --git https://github.com/rust-lang-ja/atcoder-rust-base --branch ja
	echo "cd {id}; rm -rf .git"

project.clean:
	rm -rf **/target

project.util:
	rm -rf abc088-b/.git abc088-b/target
	ln -s abc088-b/target target
	cd abc088-b && cargo clean -p abc088-b
