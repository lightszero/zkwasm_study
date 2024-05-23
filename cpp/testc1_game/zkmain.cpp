#include <stdint.h>



#include "game.h"
extern uint64_t wasm_input(uint32_t);
extern void require(int cond);
extern void wasm_dbg(uint64_t v);

__attribute__((visibility("default")))
extern "C"
int zkmain() {
    uint64_t counta = (int)wasm_input(0);
    uint64_t countb = (int)wasm_input(1);
    
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
    int outcount;
    uint64_t* outarr;
    //计算过程
    Game(counta,arra, &outcount,&outarr);

    //比对Game输出
    require(outcount == countb);
    for (int j = 0; j <countb; j++) {
        require(outarr[j] == arrb[j]);
    }
    require(1);
    return 0;
}