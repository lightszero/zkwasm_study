import * as game from "./game"

@external("env", "wasm_input")
declare function wasm_input(x: i32): i64


@external("env", "require")
declare function require(x: i32): void

export function abort(message: string | null, fileName: string | null, lineNumber: u32, columnNumber: u32): void {
    // 实现细节
    let a=0;
    a++;
}
export function zkmain(): void {
    var arra: i64[] = [];
    var arrb: i64[] = [];
    //read input
    {
        let count = wasm_input(0);

        for (var i = 0; i < count; i++) {
            let v = wasm_input(0);
            arra.push(v);
        }

    }
    {
        let countb = wasm_input(1);

        for (var i2 = 0; i2 < countb; i2++) {
            let v = wasm_input(1);
            arrb.push(v);
        }
    }
    //do game
    var arrtestb = game.game(arra);

    //check b==b`
    require(arrtestb.length == arrb.length)
    for (var j = 0; j < arrb.length; j++) {
        require(arrtestb[j] == arrb[j])
    }
    //succ
}