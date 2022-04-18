# malloc: outputs the malloc.s
# add1:   generates add1.wasm
# copy:   copy the required files into the required places

WASM_OUTPUTS = add1.wasm sum_reduce.wasm sum_reduce-noopt.wasm

all: clean add1.wat malloc copy

clean:
	rm -f *.wat
	rm -f *.wasm
	rm -f *.s

malloc:
	gcc -S -fverbose-asm -o malloc.s malloc.c

sum_reduce-noopt.wasm:
	clang --target=wasm32 \
    -nostdlib \
		-Wl,--no-entry \
		-Wl,--export-all \
		-o $@ \
		sum_reduce.c

%.wasm:
	clang --target=wasm32 \
    -nostdlib \
		-O3 \
		-flto \
		-Wl,--no-entry \
		-Wl,--export-all \
		-Wl,--lto-O3 \
		-o $@ \
		$(basename $@).c

%.wat: $(basename %).wasm
	wasm2wat $(basename $@).wasm > $@

copy: $(WASM_OUTPUTS)
	cp $^ ../public/frame/
