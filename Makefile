.PHONY: run
run: build
	@cargo run --bin rlisp -- ${ARG}
	@cargo run --bin rcwt -- ${ARG}

.PHONY: run2
run2: build
	@java -classpath javaout driver/Driver ${ARG}
	@cargo run --bin rcwt -- ${ARG}

.PHONY: build
build:
	@javac -sourcepath incremental/src -d javaout incremental/src/driver/Driver.java
	@cargo build

.PHONY: clean
clean:
	@rm -f examples/*.ast
	@rm -f examples/*.ir
	@rm -f examples/*.wc
	@rm -f examples/*.s
	@rm -f -r tmp
