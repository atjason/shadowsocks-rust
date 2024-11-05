use rc4::Rc4;
use md5::{Md5, Digest};
use std::io::{self, Read, Write};

pub struct Rc4Md5Cipher {
    rc4_cipher: Rc4,
}

impl Rc4Md5Cipher {
    pub fn new(password: &str) -> Self {
        // Generate MD5 hash from the password
        let md5_hash = Md5::digest(password.as_bytes());
        // Initialize RC4 cipher with MD5 hash as the key
        let rc4_cipher = Rc4::new(&md5_hash).unwrap();
        Rc4Md5Cipher { rc4_cipher }
    }
}

impl Read for Rc4Md5Cipher {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.rc4_cipher.apply_keystream(buf);
        Ok(buf.len())
    }
}

impl Write for Rc4Md5Cipher {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.rc4_cipher.apply_keystream(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
