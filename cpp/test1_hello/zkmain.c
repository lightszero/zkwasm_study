#include <stdint.h>

extern uint64_t wasm_input(uint32_t);
extern void require(int cond);
extern void wasm_dbg(uint64_t v);

__attribute__((visibility("default")))
int zkmain() {
    uint64_t a = wasm_input(0);
    uint64_t b = wasm_input(1);
    require(a==b);

    return 0;
}