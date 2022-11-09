#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>


#include "../falcon.h"

#define MY_FALCON_DEGREE 512
#define MY_FALCON_LOGN 9
// 512 = 9eme bit

int error_type(int ret_value_function);

int main(){

    int private_key_size;
    int public_key_size;
    int padded_signature_size;
    int tmp_keygen_size;
    int tmp_signature_size;
    shake256_context shake_ct;
    int i;

    private_key_size = FALCON_PRIVKEY_SIZE(MY_FALCON_LOGN);
    public_key_size = FALCON_PUBKEY_SIZE(MY_FALCON_LOGN);
    padded_signature_size = FALCON_SIG_PADDED_SIZE(MY_FALCON_LOGN);
    tmp_keygen_size = FALCON_TMPSIZE_KEYGEN(MY_FALCON_LOGN);
    tmp_signature_size = FALCON_TMPSIZE_SIGNDYN(MY_FALCON_LOGN);

    printf("Private key size is : %d\n", private_key_size);
    printf("Public key size is : %d\n", public_key_size);
    printf("Padded signature size is : %d\n", padded_signature_size);

    
    // SHAKE-256 Random initialization
    error_type(shake256_init_prng_from_system(&shake_ct));
    
    // Falcon keygen variable
    unsigned char tmp_keygen[tmp_keygen_size];
    unsigned char private_key[private_key_size];
    unsigned char public_key[public_key_size];

    // Falcon keygen operations
    if(!error_type(falcon_keygen_make(&shake_ct, MY_FALCON_LOGN, private_key,
    private_key_size, public_key, public_key_size, tmp_keygen, tmp_keygen_size))){
        printf("Public key :\n");
        for(i=0; i<public_key_size; i++){
            printf("%x", public_key[i]);
        }
        printf("\n");
    }

    // SHAKE-256 Random reinitialization
    error_type(shake256_init_prng_from_system(&shake_ct));

    // Falcon signature variable
    unsigned char string_to_sign[40] = "Big Bert is coming Motherfucker !!!";
    int string_to_sign_size = 37;
    unsigned char padded_signature[padded_signature_size];
    unsigned char tmp_signature[tmp_signature_size];

    // Falcon signature operations
    if(!error_type(falcon_sign_dyn(&shake_ct, padded_signature, &padded_signature_size,
    FALCON_SIG_PADDED, private_key, private_key_size, string_to_sign, 
    string_to_sign_size, tmp_signature, tmp_signature_size))){
        printf("Signature :\n");
        for(i=0; i<padded_signature_size; i++){
            printf("%x", padded_signature[i]);
        }
        printf("\n");
    }
    

    // Falcon verification variable
    int tmp_verify_size = FALCON_TMPSIZE_VERIFY(MY_FALCON_LOGN);
    unsigned char tmp_verify[tmp_verify_size];

    // Falcon verification operations
    if(!error_type(falcon_verify(padded_signature, padded_signature_size, FALCON_SIG_PADDED,
    public_key, public_key_size, string_to_sign, string_to_sign_size, 
    tmp_verify, tmp_verify_size))){
        printf("Signature 1 = OK !!\n");
    }

    // Modification to create an error
    public_key[16] = "c";
    printf("Creation of an error by modifying a byte of the public key...\n");

    // Falcon verification error
    if(!error_type(falcon_verify(padded_signature, padded_signature_size, FALCON_SIG_PADDED,
    public_key, public_key_size, string_to_sign, string_to_sign_size, 
    tmp_verify, tmp_verify_size))){
        printf("Signature = OK !!\n");
    }
    return 0;

}



int error_type(int ret_value_function){
    switch (ret_value_function){
        case 0:
            return 0;
        case -1:
            printf("RNG Error : RNG failed or none is supported.\n");
            exit(-1);
        case -2:
            printf("Size Error : Buffer size is too small to receive the intented value.\n");
            exit(-2);
        case -3:
            printf("Format Error : Decoding failed.\n");
            exit(-3);
        case -4:
            printf("Bad Signature Error : Signature does not match the provided message and public key.\n");
            exit(-4);
        case -5:
            printf("Bad Argument Error : Provided parameter is not in valid range.\n");
            exit(-5);
        case -6:
            printf("Internal Error : Falcon internal computation failed.\n");
            exit(-6);
            
    }
}
    


