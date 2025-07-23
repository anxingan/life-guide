SHELL := /bin/bash
.DEFAULT_GOAL := help

###########################
# VARIABLES
###########################

.PHONY: help
help:  ## help target to show available commands with information
	@echo "Usage:"
	@echo "  make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk '{split($$0,a,":.*?## "); printf " %-15s %s\n", a[1], a[2]}'

.PHONY: serve
serve:  ## serve target to start mdbook server
	mdbook serve -o
