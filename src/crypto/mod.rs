use crate::crypto::rc4_md5::Rc4Md5Cipher;

pub fn get_cipher(method: &str, password: &str) -> Box<dyn Cipher> {
    match method {
        "rc4-md5" => Box::new(Rc4Md5Cipher::new(password)),
        // Add other cipher methods here as needed
        _ => unimplemented!("Cipher method {} is not supported.", method),
    }
}
