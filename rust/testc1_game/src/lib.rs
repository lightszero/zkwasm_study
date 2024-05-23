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
    unsafe
    {
        let a:u64 = wasm_input(0);
        let b:u64 = wasm_input(1);
        require(a==b);
    }
    return 0;
}