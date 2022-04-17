# malloc: outputs the malloc.s
# add1:   generates add1.wasm
# copy:   copy the required files into the required places

all: add1 malloc copy

malloc:
	gcc -S -fverbose-asm -o malloc.s malloc.c

# nostblib = don't try linking libc
add1:
	clang --target=wasm32 \
    -nostdlib \
		-O3 \
		-flto \
		-Wl,--no-entry \
		-Wl,--export-all \
		-Wl,--lto-O3 \
		-o add1.wasm \
		add1.c

copy:
	cp ./add1.wasm ../public/frame/add1.wasm
