.PHONY: run
run: build
	@cargo run --bin rlisp -- ${ARG}
	@cargo run --bin rcwt -- ${ARG}

.PHONY: build
build:
	@cargo build

.PHONY: clean
clean:
	@rm -f -r tgt
	@rm -f -r tmp
