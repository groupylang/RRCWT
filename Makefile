ifeq ($(OS), Windows_NT)
.PHONY: build
build:
	@javac -sourcepath incremental/src -d javaout incremental/src/driver/Driver.java
	@cargo build
	@copy target/debug/driver.exe rrcwt.exe
.PHONY: clean
clean:
	@rmdir /q /s tmp
	@del /F /Q rrcwt.exe
else
.PHONY: build
build:
	@javac -sourcepath incremental/src -d javaout incremental/src/driver/Driver.java
	@cargo build
	@cp target/debug/driver rrcwt
.PHONY: clean
clean:
	@rm -f -r tmp
	@rm -f rrcwt
endif
