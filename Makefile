.PHONY: run
run: build
	@java -classpath javaout driver/Driver ${ARG}
	@cargo run --bin rcwt -- tmp/${ARG}

.PHONY: run2
run2: build
	@cargo run --bin rlisp -- ${ARG}
	@cargo run --bin rcwt -- tmp/${ARG}

.PHONY: build
build:
	@javac -sourcepath incremental/src -d javaout incremental/src/driver/Driver.java
	@cargo build

.PHONY: clean
clean:
	@rm -f -r tmp
