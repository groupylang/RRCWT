ifeq ($(OS), Windows_NT)
RRCWT = rrcwt.bat
_RRCWT = rrcwt.bat
MD = md
RD = rd /q /s
RM = del /Q
else
RRCWT = ./rrcwt
_RRCWT = rrcwt
MD = mkdir -p
RD = rm -f -r
RM = rm -f
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
	$(RD) tmp
	$(RM) $(_RRCWT)

test:
	$(RRCWT) direct examples/arith
	$(RRCWT) direct examples/logic
	$(RRCWT) direct examples/array
	$(RRCWT) direct examples/fn

lib: driver/lib/hello.cpp driver/lib/io.cpp
ifeq ($(OS), Windows_NT)
	clang++ driver/lib/hello.cpp driver/lib/io.cpp -shared -fPIC -Wall -o rcwtlib.dll
else
	clang++ driver/lib/hello.cpp driver/lib/io.cpp -shared -fPIC -Wall -o rcwtlib.so
endif