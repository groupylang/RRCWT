.SILENT: prepare build clean build_java build_rust
.PHONY: prepare build clean build_java build_rust

prepare:
ifeq ($(OS), Windows_NT)
	-md javaout > NUL 2>&1
else
	mkdir -p javaout
endif
	
build: prepare build_java build_rust
ifeq ($(OS), Windows_NT)
	-echo target\debug\driver.exe %%*> rrcwt.bat
else
	cp target/debug/driver rrcwt
endif
clean:
ifeq ($(OS), Windows_NT)
	-rd /q /s tmp
	-del /Q rrcwt.bat
else
	rm -f -r tmp
	rm -f rrcwt
endif

build_java:
	javac -sourcepath incremental/src -d javaout incremental/src/driver/Driver.java
build_rust:
	cargo build --verbose
