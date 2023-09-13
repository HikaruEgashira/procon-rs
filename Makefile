SHELL := /usr/bin/env bash

all:

setup:
	@cargo install cargo-atcoder --force
	@cargo atcoder login
	@cp cargo-atcoder.toml ~/.cargo/cargo-atcoder.toml

clean:
	fd target --no-ignore --exec rm -rf

code:
	DIR="$$(fd cargo.toml --exec echo {//} | fzf)"; \
	code -r "$$DIR"

new:
	cd atcoder.jp/contests && \
	cargo atcoder new $(filter-out $@,$(MAKECMDGOALS)) && \
	code -r $(filter-out $@,$(MAKECMDGOALS))
