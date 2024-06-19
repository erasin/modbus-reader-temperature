# Default target when `make` is run without arguments.
.DEFAULT_GOAL := help
width=10

# Generate a list of available commands and their descriptions by parsing the Makefile.
help:
	@echo "命令:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%${width}s\033[0m: %s\n", $$1, $$2}'
	@echo ""

build: ## build
	@cargo build --package mb

mock: ## mock
	@cargo run --package mb-mock

read: ## reader
	@cargo run --package mb-reader

clean: ## clean cache files
	@cargo clean

# Special targets that are not files:
.PHONY: help
