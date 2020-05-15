ifeq ($(OS), Windows_NT)
RRCWT = rrcwt.bat
_RRCWT = rrcwt.bat
MD = md
RD = rd /q /s
RM = del /Q
DLL = dll
else
RRCWT = ./rrcwt
_RRCWT = rrcwt
MD = mkdir -p
RD = rm -f -r
RM = rm -f
DLL = so
endif

.PHONY:build clean test
	
build:
	-$(MD) javaout
	javac -sourcepath incremental/src -d javaout incremental/src/driver/Driver.java
	cargo build --verbose
ifeq ($(OS), Windows_NT)
	-echo target\debug\driver.exe %%*> rrcwt.bat
else
	cp target/debug/driver rrcwt
endif

clean:
	-$(RD) tmp
	-$(RM) $(_RRCWT)
	-$(RM) rcwtlib.$(DLL)

test:
	cargo test --verbose
	$(RRCWT) direct examples/arith
	$(RRCWT) direct examples/logic
	$(RRCWT) direct examples/array
	$(RRCWT) direct examples/fn
	$(RRCWT) direct examples/loop
	$(RRCWT) direct examples/float

lib: rcwt/src/c/env.cpp driver/lib/hello.cpp driver/lib/io.cpp driver/lib/hash.cpp
	clang++ rcwt/src/c/env.cpp driver/lib/hello.cpp driver/lib/io.cpp driver/lib/hash.cpp -shared -fPIC -Wall -o rcwtlib.$(DLL)