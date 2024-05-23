#include <stdint.h>

extern uint64_t wasm_input(uint32_t);
extern void require(int cond);
extern void wasm_dbg(uint64_t v);

__attribute__((visibility("default")))
int zkmain() {
    uint64_t counta = wasm_input(0);
    uint64_t countb = wasm_input(1);
    
    //read input
    uint64_t* arra = new  uint64_t[counta];
    uint64_t* arrb = new  uint64_t[countb];
    {
        for (int i = 0; i < counta; i++) {
            auto v = wasm_input(0);
            arra[i]=v;
        }

    }
    {
        for (int i = 0; i < countb; i++) {
            auto v = wasm_input(1);
            arrb[i]=v;
        }
    }
    require(1);
    return 0;
}