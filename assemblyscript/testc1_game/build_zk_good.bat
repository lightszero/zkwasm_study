
asc zk.ts -O --noAssert -o ..\bin\zk_testc1.wasm --disable bulk-memory  --runtime stub --use abort=0 

# --disable bulk-memory  数组不会分配了，指令正常
# --use abort=0          可以解决abort 不能用问题


