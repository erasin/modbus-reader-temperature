# Default target when `make` is run without arguments.
.DEFAULT_GOAL := help
width=10

# Generate a list of available commands and their descriptions by parsing the Makefile.
help:
	@echo "命令:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%${width}s\033[0m: %s\n", $$1, $$2}'
	@echo ""

build: ## build
	@cargo build 

mock: ## mock
	@cargo run --package mb-mock

read: ## reader
	@cargo run --package mb-read


gui:build ## build and gui
	@cd mb-reader-gui && godot

win: ## window
	cargo build --package mb-gd --release --target i686-pc-windows-gnu; \
    cargo build --package mb-gd --release --target x86_64-pc-windows-gnu

clean: ## clean cache files
	@cargo clean

# Special targets that are not files:
.PHONY: help
