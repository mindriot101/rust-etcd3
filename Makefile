.PHONY: all
all: rust


.PHONY: rust
rust:
	-docker-compose run --user $(shell id -u):$(shell id -g) --rm rust
