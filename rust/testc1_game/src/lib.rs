mod game;
use wasm_bindgen::prelude::*;


extern "C" {
    pub fn wasm_input(is_public: u32) -> u64;
    pub fn wasm_output(v: u64);
    pub fn wasm_read_context() -> u64;
    pub fn wasm_write_context(v: u64);
    pub fn require(cond: bool);
    pub fn wasm_dbg(v: u64);
    pub fn wasm_dbg_char(v: u64);

    pub fn merkle_setroot(x: u64);
    pub fn merkle_address(x: u64);
    pub fn merkle_set(x: u64);
    pub fn merkle_get() -> u64;
    pub fn merkle_getroot() -> u64;
    pub fn merkle_fetch_data() -> u64;
    pub fn merkle_put_data(x: u64);
    pub fn poseidon_new(x: u64);
    pub fn poseidon_push(x: u64);
    pub fn poseidon_finalize() -> u64;

    pub fn babyjubjub_sum_new(x: u64);
    pub fn babyjubjub_sum_push(x: u64);
    pub fn babyjubjub_sum_finalize() -> u64;

    /// inserts a witness at the current wasm_private inputs cursor
    pub fn wasm_witness_insert(u: u64);
    pub fn wasm_witness_pop() -> u64;
    pub fn wasm_witness_set_index(x: u64);
    pub fn wasm_witness_indexed_pop() -> u64;
    pub fn wasm_witness_indexed_insert(x: u64);
    pub fn wasm_witness_indexed_push(x: u64);
}


#[wasm_bindgen]
pub fn zkmain() -> i64 {
    let mut _arra: [u64; 1024]=[0; 1024];
    let mut _arrb: [u64; 1024]=[0; 1024];;
    let mut _arrbout: [u64; 1024]=[0; 1024];
    let mut _arrbcount:usize = 0;
    unsafe
    {
        let counta = wasm_input(0) as usize;
        let countb = wasm_input(1) as usize;
        for i in 0..counta        {
            _arra[i] = wasm_input(0);
        }
        for i in 0..countb        {
            _arrb[i] = wasm_input(0);
        }
        game::DoGame(counta,&_arra as *const u64,&mut _arrbcount ,&mut _arrbout as *mut u64);
     
        for i in 0..countb        {
            require( _arrb[i] == _arrbout[i]);
        }
    }
    return 0;
}