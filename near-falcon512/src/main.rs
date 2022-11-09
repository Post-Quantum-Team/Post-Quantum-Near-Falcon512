
pub mod ffi;
use std::u8;

use crate::ffi::*;




struct Shake256Context (pub [u64; SHAKE256_CONTEXT_SIZE]);
//struct Falcon512PublicKey (pub[u8; ])

fn main() {
    let mut test = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
    unsafe { shake256_init(test.0.as_mut_ptr()) }
    let seed = "ceci est la seed";
    let seed_size = seed.len();
    println!("La taille est de : {}.", seed_size);
    unsafe {
        shake256_init_prng_from_seed(test.0.as_mut_ptr(), seed.as_ptr(), seed_size);
    }
    let mut output = [0u8; 128];
    unsafe {
        shake256_extract(test.0.as_mut_ptr(), output.as_mut_ptr(), 128);
    }
    //let output = String::from(output.);
    println!("{}", hex::encode(output));

    // Get Keypair Falcon elements size
    println!("pub key size = {}", NEAR_FALCON512_PUBKEY_SIZE);
    println!("Private key size = {}", NEAR_FALCON512_PRIVKEY_SIZE);
    println!("Tmp keygen size = {}", NEAR_FALCON512_TMPSIZE_KEYGEN);

    // Initialize Keypair elements
    let mut pk = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
    let mut sk = [0u8; NEAR_FALCON512_PRIVKEY_SIZE];
    let mut sig = [0u8; NEAR_FALCON512_SIG_PADDED_SIZE];
    let mut tmp_keygen = [0u8; NEAR_FALCON512_TMPSIZE_KEYGEN];
    let mut tmp_makepub = [0u8; NEAR_FALCON512_TMPSIZE_MAKEPUB];
    let mut tmp_signdyn = [0u8; NEAR_FALCON512_TMPSIZE_SIGNDYN];
    let mut tmp_verify = [0u8; NEAR_FALCON512_TMPSIZE_VERIFY];



    unsafe{ 
        let result = falcon_keygen_make(test.0.as_ptr(), NEAR_FALCON_DEGREE, sk.as_mut_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, pk.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_KEYGEN);
        println!("The result of keypair generation (0 if success) is : {}", result);

        println!("{}", hex::encode(pk));

        // regenerate pubkey
        let mut pk = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
        let result = falcon_make_public(pk.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, sk.as_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, tmp_makepub.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_MAKEPUB);
        println!("The result of REGENERATED pubkey (0 if success) is : {}", result);
        println!("Regenerated public key : {}", hex::encode(pk));

        shake256_init_prng_from_system(test.0.as_mut_ptr());

        let text = "Bonjour ceci est un petit test";
        let text_size = text.len();
        let sig_len = NEAR_FALCON512_SIG_PADDED_SIZE;
        let result = falcon_sign_dyn(test.0.as_ptr(), sig.as_mut_ptr(), &sig_len, FALCON_SIG_PADDED, sk.as_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, text.as_ptr(), text_size, tmp_signdyn.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_SIGNDYN);
        println!("The result of dynamic signature (0 if success) is : {}", result);

        let result = falcon_verify(sig.as_ptr(), sig_len, FALCON_SIG_PADDED, pk.as_ptr(), NEAR_FALCON512_PUBKEY_SIZE, text.as_ptr(), text_size, tmp_verify.as_ptr(), NEAR_FALCON512_TMPSIZE_VERIFY);
        //(test.0.as_ptr(), sig.as_mut_ptr(), &sig_len, FALCON_SIG_PADDED, sk.as_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, text.as_ptr(), text_size, tmp_signdyn.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_SIGNDYN);
        println!("The result of signature verification (0 if success) is : {}", result);
        



        



    } ;


    //println!("{}", test.0);
}