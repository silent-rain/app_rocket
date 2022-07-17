/*!aes 加密/解密
 *
*/
use std::error::Error;

use crypto::aes::KeySize::KeySize128;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};

/// 加密
/// 16, 24, or 32 字节的 key 对应 KeySize128, KeySize192, or KeySize256
fn encrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut encrypt = crypto::aes::ecb_encryptor(KeySize128, key, PkcsPadding);
    let mut read_buffer = RefReadBuffer::new(text);
    let mut result = vec![0; text.len() * 4];
    let mut write_buffer = RefWriteBuffer::new(&mut result);
    encrypt
        .encrypt(&mut read_buffer, &mut write_buffer, true)
        .unwrap();
    Ok(result.into_iter().filter(|v| *v != 0).collect())
}

/// 解密
fn decrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decrypt = crypto::aes::ecb_decryptor(KeySize128, key, PkcsPadding);
    let mut read_buffer = RefReadBuffer::new(text);

    let mut result = vec![0; text.len()];
    let mut write_buffer = RefWriteBuffer::new(&mut result);

    decrypt
        .decrypt(&mut read_buffer, &mut write_buffer, true)
        .map_err(|e| format!("{:?}", e))?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64;

    #[test]
    fn test_raw() {
        let key = "ix4En7l1Hau10aPq".as_bytes();
        let text = "123132ikjjigui".as_bytes();
        // let text = b"\xE8\xBF\x99\xE6\x98\xAF\xE4\xB8\x80\xE4\xB8\xAA\xE6\xB5\x8B\xE8\xAF\x95";
        println!("原文: {:?} === {}", &text, &text.len());

        // 加密
        let encrypt_text = encrypt(key, text).unwrap();
        println!("密文: {:?} === {}", &encrypt_text, &encrypt_text.len());

        // 解密
        let decrypt_text = decrypt(key, &encrypt_text).unwrap();
        let text = String::from_utf8(decrypt_text).unwrap();
        println!("明文: {}", text);

        // 9qwNiXZCDwBTFZbjVNfp/GZ4Rt/8ZZhQ6B6APMybHKM=
        // [246, 172, 13, 137, 118, 66, 15, 83, 21, 150, 227, 84, 215, 233, 252, 102, 120, 70, 223, 252, 101, 152, 80, 232, 30, 128, 60, 204, 155, 28, 163]
        // [215, 107, 146, 154, 111, 116, 206, 251, 11, 2, 75, 191, 156, 199, 130, 55]
    }

    #[test]
    fn test_base64() {
        let key = "ix4En7l1Hau10aPq".as_bytes();
        let text = "123132ikjjigui".as_bytes();
        // 加密
        let encrypt_text = encrypt(key, text).unwrap();
        let encode_text = base64::encode(&encrypt_text);
        println!("密文: {:?}", encode_text);

        // 解密
        let decode_text = base64::decode(encode_text).unwrap();
        let decrypt_text = decrypt(key, &decode_text).unwrap();
        let text = String::from_utf8(decrypt_text).unwrap();
        println!("明文: {}", text);
    }
    #[test]
    fn test_hex() {
        let key = b"ix4En7l1Hau10aPq";
        let text = "123132ikjjigui".as_bytes();
        // 加密
        let encrypt_text = encrypt(key, text).unwrap();
        let encode_test = hex::encode(encrypt_text);
        println!("密文: {}", encode_test);

        // 解密
        let text2 = encode_test;
        let hex_decode = hex::decode(text2).unwrap();
        let decrypt_text = decrypt(key, &hex_decode).unwrap();
        let text = String::from_utf8(decrypt_text).unwrap();
        println!("明文: {}", text);
    }
}
