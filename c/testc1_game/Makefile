LIBS  = -lkernel32 -luser32 -lgdi32 -lopengl32
#SDK_DIR = ../../sdk
CFLAGS = -Wall 
#-I$(SDK_DIR)/c/sdk/include/ -I$(SDK_DIR)/c/hash/include/ -I$(SDK_DIR)/c/rlp/include/

# Should be equivalent to your list of C files, if you don't build selectively
CFILES = $(wildcard *.c)
ifeq ($(CLANG),)
CLANG=clang-15
endif
FLAGS = -flto -O3 -nostdlib -fno-builtin -ffreestanding -mexec-model=reactor --target=wasm32 -Wl,--strip-all -Wl,--initial-memory=131072 -Wl,--max-memory=131072 -Wl,--no-entry -Wl,--allow-undefined -Wl,--export-dynamic

all: output.wasm

#sdk.wasm:
#	sh $(SDK_DIR)/scripts/build.sh sdk.wasm

output.wasm: $(CFILES ) 
	$(CLANG) -o $@ $(CFILES) $(FLAGS) $(CFLAGS)


clean:
	rm -f *.wasm *.wat
