.SILENT: build clean build_java build_rust
.PHONY: build clean build_java build_rust

build: build_java build_rust
ifeq ($(OS), Windows_NT)
	copy target/debug/driver.exe rrcwt.exe
else
	cp target/debug/driver rrcwt
endif
clean:
ifeq ($(OS), Windows_NT)
	rmdir /q /s tmp
	del /F /Q rrcwt.exe
else
	rm -f -r tmp
	rm -f rrcwt
endif

build_java:
	javac -sourcepath incremental/src -d javaout incremental/src/driver/Driver.java
build_rust:
	cargo build
