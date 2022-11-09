## Documentation
### Secret key specific
A private key, in its encoded format, can be used as parameter to falcon_sign_dyn(). 

An "expanded private key" is computed with falcon_expand_privkey(), to be used with falcon_sign_tree(). 

The expanded private key is much larger than the encoded private key, and its format is not portable. Its size (in bytes) is provided by FALCON_EXPANDEDKEY_SIZE. There are no specific alignment requirements on expanded keys, except that the alignment of a given expanded key must not change (i.e. if an expanded key is moved from address addr1 to address addr2, then it must hold that addr1 = addr2 mod 8).

Expanded private keys are meant to be used when several signatures are to be computed with the same private key: amortized cost per signature is about halved when using expanded private keys (for short messages, and depending on underlying architecture and implementation choices).

#### Blockchain project application
Function that will be used will likely be **falcon_sign_dyn()**, because the number of signature for a single user corresponds to the number of transactions of that user. 10 transactions in 10 minutes is already a good amount.

The signature is computed only once per transaction.

### Error codes
Most functions in this API that may fail for some reason return a 'int' value which will be 0 on success, or a negative error code.


|Code erreur|Dénomination|Explication|
|---|---|---|
|-1|FALCON_ERR_RANDOM|is returned when the library tries to use an OS-provided RNG, but either none is supported, or that RNG fails.|
|-2|FALCON_ERR_SIZE|is returned when a buffer has been provided to the library but is too small to receive the intended value.|
|-3|FALCON_ERR_FORMAT|is returned when decoding of an external object (public key, private key, signature) fails.|
|-4|FALCON_ERR_BADSIG|is returned when verifying a signature, the signature is validly encoded, but its value does not match the provided message and public key.|
|-5|FALCON_ERR_BADARG|is returned when a provided parameter is not in a valid range.|
|-6|FALCON_ERR_INTERNAL|is returned when some internal computation failed.|


### Signature formats
|Valeur|Dénomination|Type|Description|
|---|---|---|---|
|1|FALCON_SIG_COMPRESSED|**Variable-size signature**| This format produces the most compact signatures on average, but the signature size may vary depending on private key, signed data, and random seed.|
|2|FALCON_SIG_PADDED|**Fixed-size signature**| This format produces is equivalent to the "compressed" format, but includes padding to a known fixed size (specified by FALCON_SIG_PADDED_SIZE). With this format, the signature generation loops until an appropriate signature size is achieved (such looping is uncommon) and adds the padding bytes; the verification functions check the presence and contents of the padding bytes.|
|3|FALCON_SIG_CT|**Fixed-size format amenable to constant-time implementation**|All formats allow constant-time code with regard to the private key; the 'CT' format of signature also prevents information about the signature value and the signed data hash to leak through timing-based side channels (this feature is rarely needed).|

### Sizes
The sizes are expressed in bytes. Each size depends on the Falcon degree, which is provided logarithmically: use logn=9 for Falcon-512, logn=10 for Falcon-1024. Valid values for logn range from 1 to 10 (values 1 to 8 correspond to reduced variants of Falcon that do not provided adequate security and are meant for research purposes only).

|Logn|Taille|
|---|---|
|9|512|
|10|1024|

The sizes are provided as macros that evaluate to constant expressions, as long as the 'logn' parameter is itself a constant expression. Moreover, all sizes are monotonic (for each size category, increasing logn cannot result in a shorter length).

Note: each macro may evaluate its argument 'logn' several times.

|Element|Description|Macro|
|---|---|---|
|Private Key|Private key size (in bytes). The size is exact.|FALCON_PRIVKEY_SIZE(logn)|
|Public Key|Public key size (in bytes). The size is exact.|FALCON_PUBKEY_SIZE(logn)|
|Compressed format signature|Maximum signature size (in bytes) when using the COMPRESSED format. In practice, the signature will be shorter.|FALCON_SIG_COMPRESSED_MAXSIZE(logn)|
|Padded format signature|Signature size (in bytes) when using the PADDED format. The size is exact.|FALCON_SIG_PADDED_SIZE(logn)|
|CT format signature|Signature size (in bytes) when using the CT format. The size is exact.|FALCON_SIG_CT_SIZE(logn)|
|Keypair generation|Temporary buffer size for key pair generation.|FALCON_TMPSIZE_KEYGEN(logn)|
|Public Key Computation|Temporary buffer size for computing the pubic key from the private key.|FALCON_TMPSIZE_MAKEPUB(logn)|
|Dynamique Signature Generation|Temporary buffer size for generating a signature ("dynamic" variant).|FALCON_TMPSIZE_SIGNDYN(logn)|
|Tree Signature Generation|Temporary buffer size for generating a signature ("tree" variant, with an expanded key).|FALCON_TMPSIZE_SIGNTREE(logn)|
|Private Key Expanding|Temporary buffer size for expanding a private key.|FALCON_TMPSIZE_EXPANDPRIV(logn)|
|Expanded Private Key|Size of an expanded private key.|FALCON_EXPANDEDKEY_SIZE(logn)|
|Signature verification|Temporary buffer size for verifying a signature.|FALCON_TMPSIZE_VERIFY(logn)|

### SHAKE256
SHAKE256 is used in two places:

    - As a PRNG: all functions that require randomness (key pair generation, signature generation) receive as parameter a SHAKE256 object, in output mode, from which pseudorandom data is obtained.
        A SHAKE256 instance, to be used as a RNG, can be initialized from an explicit 48-byte seed, or from an OS-provided RNG. Using an explicit seed is meant for reproducibility of test vectors, or to be used in cases where no OS-provided RNG is available and supported.
 
    - As the hashing mechanism for the message which should be signed.
        The streamed signature API exposes that SHAKE256 object, since the caller then performs the hashing externally.


#### Structures
Context for a SHAKE256 computation. Contents are opaque.
Contents are pure data with no pointer; they need not be released explicitly and don't reference any other allocated resource. The caller is responsible for allocating the context structure itself, typically on the stack.

    typedef struct {
        uint64_t opaque_contents[26];
    } shake256_context;


#### Functions

|Function|Description|
|---|---|
|void shake256_init(shake256_context *sc);|Initialize a SHAKE256 context to its initial state. The state is then ready to receive data (with shake256_inject()).|
|void shake256_inject(shake256_context *sc, const void *data, size_t len);|Inject some data bytes into the SHAKE256 context ("absorb" operation). This function can be called several times, to inject several chunks of data of arbitrary length.|
|void shake256_flip(shake256_context *sc);| Flip the SHAKE256 state to output mode. After this call, shake256_inject() can no longer be called on the context, but shake256_extract() can be called. Flipping is one-way; a given context can be converted back to input mode only by initializing it again, which forgets all previously injected data.|
|void shake256_extract(shake256_context *sc, void *out, size_t len);|Extract bytes from the SHAKE256 context ("squeeze" operation). The context must have been flipped to output mode (with shake256_flip()). Arbitrary amounts of data can be extracted, in one or several calls to this function.|
|void shake256_init_prng_from_seed(shake256_context *sc,const void *seed, size_t seed_len);|Initialize a SHAKE256 context as a PRNG from the provided seed. This initializes the context, injects the seed, then flips the context to output mode to make it ready to produce bytes.|
|int shake256_init_prng_from_system(shake256_context *sc);|Initialize a SHAKE256 context as a PRNG, using an initial seed from the OS-provided RNG. If there is no known/supported OS-provided RNG, or if that RNG fails, then the context is not properly initialized and FALCON_ERR_RANDOM is returned. <br />**Returned value: 0 on success, or a negative error code.**|

### Keypair Generation

#### Functions

|Function|Description|
|---|---|
|int falcon_keygen_make(shake256_context \*rng, unsigned logn, void \*privkey, size_t privkey_len, void \*pubkey, size_t pubkey_len, void \*tmp, size_t tmp_len); | Generate a new keypair.<br />The logarithm of the Falcon degree (logn) must be in the 1 to 10 range; values 1 to 8 correspond to reduced versions of Falcon that do not provide adequate security and are meant for research purposes only.<br /> The source of randomness is the provided SHAKE256 context *rng, which must have been already initialized, seeded, and set to output mode (see shake256_init_prng_from_seed() and shake256_init_prng_from_system()).<br />The new private key is written in the buffer pointed to by privkey. The size of that buffer must be specified in privkey_len; if that size is too low, then this function fails with FALCON_ERR_SIZE. The actual private key length can be obtained from the FALCON_PRIVKEY_SIZE() macro.<br />If pubkey is not NULL, then the new public key is written in the buffer pointed to by pubkey. The size of that buffer must be specified in pubkey_len; if that size is too low, then this function fails with FALCON_ERR_SIZE. The actual public key length can be obtained from the FALCON_PUBKEY_SIZE() macro.<br /> If pubkey is NULL then pubkey_len is ignored; the private key will still be generated and written to privkey[], but the public key won't be written anywhere. The public key can be later on recomputed from the private key with falcon_make_public().<br /> The tmp[] buffer is used to hold temporary values. Its size tmp_len MUST be at least FALCON_TMPSIZE_KEYGEN(logn) bytes.<br /><br />**Returned value: 0 on success, or a negative error code.**|
|int falcon_make_public(void \*pubkey, size_t pubkey_len, const void \*privkey, size_t privkey_len, void \*tmp, size_t tmp_len);| Recompute the public key from the private key.<br /> The private key is provided encoded. This function decodes the private key and verifies that its length (in bytes) is exactly the provided value privkey_len (trailing extra bytes are not tolerated).<br />The public key is written in the buffer pointed to by pubkey. The size (in bytes) of the pubkey buffer must be provided in pubkey_len; if it is too short for the public key, then FALCON_ERR_SIZE is returned. The actual public key size can be obtained from the FALCON_PUBKEY_SIZE() macro.<br />The tmp\[\] buffer is used to hold temporary values. Its size tmp_len MUST be at least FALCON_TMPSIZE_MAKEPUB(logn) bytes.<br /><br />**Returned value: 0 on success, or a negative error code.**|
|int falcon_get_logn(void \*obj, size_t len);|Get the Falcon degree from an encoded private key, public key or signature. Returned value is the logarithm of the degree (1 to 10), or a negative error code.|



### Signature generation
|Function|Description|
|---|---|
|int falcon_sign_dyn(shake256_context \*rng,	void \*sig, size_t \*sig_len, int sig_type, const void \*privkey, size_t privkey_len, const void \*data, size_t data_len, void *tmp, size_t tmp_len); | Sign the data provided in buffer data[] (of length data_len bytes), using the private key held in privkey[] (of length privkey_len bytes).<br />The source of randomness is the provided SHAKE256 context \*rng, which must have been already initialized, seeded, and set to output mode (see shake256_init_prng_from_seed() and shake256_init_prng_from_system()). <br />The signature is written in sig[]. The caller must set \*sig_len to the maximum size of sig[]; if the signature computation is successful, then \*sig_len will be set to the actual length of the signature. The signature length depends on the signature type, which is specified with the sig_type parameter to one of the three defined values FALCON_SIG_COMPRESSED, FALCON_SIG_PADDED or FALCON_SIG_CT; for the last two of these, the signature length is fixed (for a given Falcon degree).<br />Regardless of the signature type, the process is constant-time with regard to the private key. When sig_type is FALCON_SIG_CT, it is also constant-time with regard to the signature value and the message data, i.e. no information on the signature and the message may be inferred from timing-related side channels.<br />The tmp[] buffer is used to hold temporary values. Its size tmp_len MUST be at least FALCON_TMPSIZE_SIGNDYN(logn) bytes.<br /><br />**Returned value: 0 on success, or a negative error code.**|
|int falcon_expand_privkey(void \*expanded_key, size_t expanded_key_len,	const void \*privkey, size_t privkey_len, void \*tmp, size_t tmp_len); | Expand a private key. The provided Falcon private key (privkey, of size privkey_len bytes) is decoded and expanded into expanded_key[].<br />The expanded_key[] buffer has size expanded_key_len, which MUST be at least FALCON_EXPANDEDKEY_SIZE(logn) bytes (where 'logn' qualifies the Falcon degree encoded in the private key and can be obtained with falcon_get_logn()). Expanded key contents have an internal, implementation-specific format. Expanded keys may be moved in RAM only if their 8-byte alignment remains unchanged.<br /> The tmp[] buffer is used to hold temporary values. Its size tmp_len MUST be at least FALCON_TMPSIZE_EXPANDPRIV(logn) bytes.<br /><br />**Returned value: 0 on success, or a negative error code.**|
|int falcon_sign_tree(shake256_context \*rng, void \*sig, size_t \*sig_len, int sig_type, const void \*expanded_key, const void \*data, size_t data_len, void \*tmp, size_t tmp_len);|Sign the data provided in buffer data[] (of length data_len bytes), using the expanded private key held in expanded_key[], as generated by falcon_expand_privkey().<br />The source of randomness is the provided SHAKE256 context \*rng, which must have been already initialized, seeded, and set to output mode (see shake256_init_prng_from_seed() and shake256_init_prng_from_system()).<br />The signature is written in sig[]. The caller must set \*sig_len to the maximum size of sig[]; if the signature computation is successful, then \*sig_len will be set to the actual length of the signature. The signature length depends on the signature type, which is specified with the sig_type parameter to one of the three defined values FALCON_SIG_COMPRESSED, FALCON_SIG_PADDED or FALCON_SIG_CT; for the last two of these, the signature length is fixed (for a given Falcon degree).<br />Regardless of the signature type, the process is constant-time with regard to the private key. When sig_type is FALCON_SIG_CT, it is also constant-time with regard to the signature value and the message data, i.e. no information on the signature and the message may be inferred from timing-related side channels.<br />The tmp[] buffer is used to hold temporary values. Its size tmp_len MUST be at least FALCON_TMPSIZE_SIGNTREE(logn) bytes.<br /><br />**Returned value: 0 on success, or a negative error code.**|

### Signature generation - streamed API
In the streamed API, the caller performs the data hashing externally. An initialization function (falcon_sign_start()) is first called; it generates and returns a random 40-byte nonce value; it also initializes a SHAKE256 context and injects the nonce value in that context. The caller must then inject the data to sign in the SHAKE256 context, and finally call falcon_sign_dyn_finish() or falcon_sign_tree_finish() to finalize the signature generation.

#### Functions
|Function|Description|
|---|---|
|int falcon_sign_start(shake256_context \*rng, void \*nonce, shake256_context \*hash_data);|Start a signature generation context.<br />A 40-byte nonce is generated and written in nonce[]. The *hash_data context is also initialized, and the nonce is injected in that context.<br />The source of randomness is the provided SHAKE256 context *rng, which must have been already initialized, seeded, and set to output mode (see shake256_init_prng_from_seed() and shake256_init_prng_from_system()).<br /><br />Returned value: 0 on success, or a negative error code.|
|int falcon_sign_dyn_finish(shake256_context \*rng, void \*sig, size_t \\*sig_len, int sig_type, const void \*privkey, size_t privkey_len, 	shake256_context \*hash_data, const void \*nonce, void \*tmp, size_t tmp_len);|Finish a signature generation operation, using the private key held in privkey[] (of length privkey_len bytes). The hashed nonce + message is provided as the SHAKE256 context \*hash_data, which must still be in input mode (i.e. not yet flipped to output mode). That context is modified in the process.<br />The nonce value (which was used at the start of the hashing process, usually as part of a falcon_sign_start() call) must be provided again, because it is encoded into the signature. The nonce length is 40 bytes.<br />The source of randomness is the provided SHAKE256 context \*rng, which must have been already initialized, seeded, and set to output mode (see shake256_init_prng_from_seed() and shake256_init_prng_from_system()).<br />The signature is written in sig[]. The caller must set \*sig_len to the maximum size of sig[]; if the signature computation is successful, then *sig_len will be set to the actual length of the signature. The signature length depends on the signature type, which is specified with the sig_type parameter to one of the three defined values FALCON_SIG_COMPRESSED, FALCON_SIG_PADDED or FALCON_SIG_CT; for the last two of these, the signature length is fixed (for a given Falcon degree).<br />Regardless of the signature type, the process is constant-time with regard to the private key. When sig_type is FALCON_SIG_CT, it is also constant-time with regard to the signature value and the message data, i.e. no information on the signature and the message may be inferred from timing-related side channels.<br />The tmp[] buffer is used to hold temporary values. Its size tmp_len MUST be at least FALCON_TMPSIZE_SIGNDYN(logn) bytes.<br /><br />**Returned value: 0 on success, or a negative error code.**|
|int falcon_sign_tree_finish(shake256_context *rng, void *sig, size_t *sig_len, int sig_type, const void *expanded_key, shake256_context *hash_data, const void *nonce, void *tmp, size_t tmp_len); |Finish a signature generation operation, using the expanded private key held in expanded_key[] (as obtained from falcon_expand_privkey()). The hashed nonce + message is provided as the SHAKE256 context *hash_data, which must still be in input mode (i.e. not yet flipped to output mode). That context is modified in the process.<br />The nonce value (which was used at the start of the hashing process, usually as part of a falcon_sign_start() call) must be provided again, because it is encoded into the signature. The nonce length is 40 bytes.<br />The source of randomness is the provided SHAKE256 context *rng, which must have been already initialized, seeded, and set to output mode (see shake256_init_prng_from_seed() and shake256_init_prng_from_system()).<br />The signature is written in sig[]. The caller must set *sig_len to the maximum size of sig[]; if the signature computation is successful, then *sig_len will be set to the actual length of the signature. The signature length depends on the signature type, which is specified with the sig_type parameter to one of the three defined values FALCON_SIG_COMPRESSED, FALCON_SIG_PADDED or FALCON_SIG_CT; for the last two of these, the signature length is fixed (for a given Falcon degree).<br />Regardless of the signature type, the process is constant-time with regard to the private key. When sig_type is FALCON_SIG_CT, it is also constant-time with regard to the signature value and the message data, i.e. no information on the signature and the message may be inferred from timing-related side channels.<br />The tmp[] buffer is used to hold temporary values. Its size tmp_len MUST be at least FALCON_TMPSIZE_SIGNTREE(logn) bytes.<br /><br />**Returned value: 0 on success, or a negative error code.**|

### Signature verification
#### Functions

|Function|Description|
|---|---|
|int falcon_verify(const void \*sig, size_t sig_len, int sig_type, const void \*pubkey, size_t pubkey_len, const void \*data, size_t data_len, void \*tmp, size_t tmp_len);|Verify the signature sig[] (of length sig_len bytes) with regards to the provided public key pubkey[] (of length pubkey_len bytes) and the message data[] (of length data_len bytes).<br />The sig_type parameter must be zero, or one of FALCON_SIG_COMPRESSED, FALCON_SIG_PADDED or FALCON_SIG_CT. The function verifies that the provided signature has the correct format. If sig_type is zero, then the signature format is inferred from the signature header byte; note that in that case, the signature is malleable (since a signature value can be transcoded to other formats).<br />The tmp[] buffer is used to hold temporary values. Its size tmp_len MUST be at least FALCON_TMPSIZE_VERIFY(logn) bytes.<br /><br />**Returned value: 0 on success, or a negative error code.**|
|int falcon_verify_start(shake256_context \*hash_data, const void \*sig, size_t sig_len);|Start a streamed signature verification. The provided SHAKE256 context *hash_data is initialized, and the nonce value (extracted from the signature) is injected into it. The caller shall then inject the message data into the SHAKE256 context, and finally call falcon_verify_finish().<br /><br />**Returned value: 0 on success, or a negative error code.**|
|int falcon_verify_finish(const void \*sig, size_t sig_len, int sig_type, const void \*pubkey, size_t pubkey_len, shake256_context \*hash_data, void *tmp, size_t tmp_len);|Finish a streamed signature verification. The signature sig[] (of length sig_len bytes) is verified against the provided public key pubkey[] (of length pubkey_len bytes) and the hashed message. The hashed message is provided as a SHAKE256 context \*hash_data; that context must have received the nonce and the message itself (usually, the context is initialized and the nonce injected as part of a falcon_verify_start() call), and still be in input mode (not yet flipped to output mode). \*hash_data is modified by the verification process.<br />The sig_type parameter must be zero, or one of FALCON_SIG_COMPRESSED, FALCON_SIG_PADDED or FALCON_SIG_CT. The function verifies that the provided signature has the correct format. If sig_type is zero, then the signature format is inferred from the signature header byte; note that in that case, the signature is malleable (since a signature value can be transcoded to other formats).<br />The tmp[] buffer is used to hold temporary values. Its size tmp_len MUST be at least FALCON_TMPSIZE_VERIFY(logn) bytes.<br /><br />**Returned value: 0 on success, or a negative error code.**|




