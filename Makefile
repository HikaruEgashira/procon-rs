SHELL := /usr/bin/env bash

all:

clean:
	fd target --no-ignore --exec rm -rf

code:
	DIR="$$(fd cargo.toml --exec echo {//} | fzf)"; \
	code -r "$$DIR"
