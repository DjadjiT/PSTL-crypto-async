extern crate crypto;

use rand::{OsRng, Rng};
use crypto::symmetriccipher::{BlockEncryptor, BlockDecryptor, BlockEncryptorX8, BlockDecryptorX8};
use crypto::aessafe::{AesSafe128Encryptor, AesSafe128Decryptor, AesSafe128EncryptorX8, AesSafe128DecryptorX8};

/* Encrypt a array of unsigned int (128 bits)
return the encrypted array as an array of unsigned int
*/
pub fn encrypt(input : &[u8], key : &[u8]) -> [u8;16] {
    let mut output =  [0u8;16];
    //initialize the Encryptor
    let encryptor = AesSafe128Encryptor::new(&key);
    encryptor.encrypt_block(&input, &mut output);
    return output
}

/* Decrypt a array of unsigned int (1 block of 128 bits)
return the decrypted array as an array of unsigned int
*/
pub fn decrypyt(input : &[u8], key : &[u8]) -> [u8;16] {
    let mut output =  [0u8;16];
    //initialize the Decryptor
    let decryptor = AesSafe128Decryptor::new(&key);
    decryptor.decrypt_block(&input, &mut output);
    return output
}

/* Encrypt a array of unsigned int (128 bytes = 8 block of 128 bits)
return the encrypted array as an array of unsigned int
*/
pub fn encrypt_x8(input : &[u8], key : &[u8]) -> [u8;128] {
    let mut output =  [1u8;128];
    //initialize the Encryptor
    let encryptor = AesSafe128EncryptorX8::new(&key);
    encryptor.encrypt_block_x8(&input, &mut output);
    return output
}

/* Decrypt a array of unsigned int (128 bytes = 8 block of 128 bits)
return the decrypted array as an array of unsigned int
*/
pub fn decrypyt_x8(input : &[u8], key : &[u8]) -> [u8;128] {
    let mut output =  [1u8;128];
    //initialize the Decryptor
    let decryptor = AesSafe128DecryptorX8::new(&key);
    decryptor.decrypt_block_x8(&input, &mut output);
    return output
}