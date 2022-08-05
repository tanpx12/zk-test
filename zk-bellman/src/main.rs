use zkutil::circom_circuit::{verify,load_proof_json_file,load_inputs_json_file,load_params_file};
use bellman_ce::pairing::{bn256::Bn256};

fn main() {
    let res: bool;
    let proof = load_proof_json_file::<Bn256>("/home/tanpx/Desktop/rust-playground/main/test/proof.json");
    let inputs = load_inputs_json_file::<Bn256>("/home/tanpx/Desktop/rust-playground/main/test/public.json");
    let params = load_params_file("/home/tanpx/Desktop/rust-playground/main/test/params.bin");
    res = verify(&params, &proof, &inputs).unwrap();
    println!("{}",res);
}
