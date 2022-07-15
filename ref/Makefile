# malloc: outputs the malloc.s
# add1:   generates add1.wasm
# copy:   copy the required files into the required places

WASM_OUTPUTS = add1.wasm sum_reduce.wasm sum_reduce-noopt.wasm

all: clean add1.wat malloc malloc_expanded copy

clean:
	rm -f *.wat
	rm -f *.wasm
	rm -f *.s
	rm -f ./malloc
	rm -f ./malloc_expanded

malloc:
	rustc malloc.rs

malloc_expanded:
	rustc malloc_expanded.rs

sum_reduce-noopt.wasm:
	rustc \
	  --target wasm32-unknown-unknown \
		-Copt-level=0 \
		-o sum_reduce-noopt.wasm \
		sum_reduce.rs

%.wasm:
	rustc \
	  --target wasm32-unknown-unknown \
		-Copt-level=3 \
		-Clto \
		-o $@ \
		$(basename $@).rs

%.wat: $(basename %).wasm
	wasm2wat $(basename $@).wasm > $@

copy: $(WASM_OUTPUTS)
	cp $^ ../public/frame/
