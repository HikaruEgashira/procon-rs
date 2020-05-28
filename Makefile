SHELL := /usr/bin/env bash

all:

project.init:
	cargo generate --git https://github.com/rust-lang-ja/atcoder-rust-base --branch ja
	echo "cd {id}; rm -rf .git"

project.clean:
	fd target --no-ignore --exec rm -rf

project.util:
	fd .git --no-ignore --hidden --type directory
	rm -rf abc088-b/.git abc088-b/target
	ln -s abc088-b/target target
	cd abc088-b && cargo clean -p abc088-b

code:
	DIR="$$(fd cargo.toml --exec echo {//} | fzf)"; \
	code "$$DIR"
