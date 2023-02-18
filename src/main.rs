mod ats_pr;

use ats_pr::bls::BLSSignature;
use ats_pr::threshold::ThresholdKeyPairs;

// Sample params for T-of-N threshold sig
const T: usize = 2;
const N: usize = 5;
const QUORUM: [usize; 2] = [0, 3];

fn main() {
    let tkp = ThresholdKeyPairs::new(N);
    // let tk = tkp.keys[0] + tkp.keys[1];

    println!("{:?}", tkp.get_X(&QUORUM.to_vec()));

    // let message_bytes: [u8; 5] = [1, 2, 3, 4, 5];
    // let sig: BLSSignature = BLSSignature::sign(&message_bytes[..], &tk.x);
    // println!("{}", sig.verify(&message_bytes[..], &tkp.keys[0].X));
}