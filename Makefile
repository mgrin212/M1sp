CC = clang
AS = as
CFLAGS = -Wall -Wextra -pedantic
LDFLAGS = -framework CoreFoundation -arch arm64

all: hello_world

hello_world: hello_world.o runtime.o
	$(CC) $(LDFLAGS) -o $@ $^

hello_world.o: hello_world.s
	$(AS) -arch arm64 -o $@ $<

runtime.o: runtime.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f hello_world *.o

.PHONY: all clean
