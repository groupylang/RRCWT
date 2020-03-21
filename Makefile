.PHONY: run
run: build
	@cargo run --bin rlisp -- ${ARG}
	@cargo run --bin rcwt -- ${ARG}

.PHONY: build
build:
	@cargo build

.PHONY: clean
clean:
	@rm -f examples/*.ast
	@rm -f examples/*.ir
	@rm -f examples/*.wc
	@rm -f examples/*.s
	@rm -f -r tmp
