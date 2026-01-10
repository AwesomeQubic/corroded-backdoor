#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Grain128 {
    LFSR: [u8; 128],
    NFSR: [u8; 128],

    key: [u8; 16],
    keysize: usize,
    ivsize: usize,
}

impl Grain128 {
    pub fn keysetup(key: &[u8], keysize: usize, ivsize: usize) -> Self {
        Self {
            key: key.try_into().unwrap(),
            keysize,
            ivsize,
            LFSR: [0u8; 128],
            NFSR: [0u8; 128],
        }
    }

    /// Load the key and perform initial clockings.
    ///
    ///  Assumptions
    /// * The key is 16 bytes and the IV is 12 bytes. The registers are loaded in the following way:
    /// * NFSR[0] = lsb of key[0]
    /// * ...
    /// * NFSR[7] = msb of key[0]
    /// * ...
    /// * NFSR[120] = lsb of key[16]
    /// * ...
    /// * NFSR[127] = msb of key[16]
    /// * LFSR[0] = lsb of IV[0]
    /// * ...
    /// * LFSR[7] = msb of IV[0]
    /// * ...
    /// * LFSR[88] = lsb of IV[12]
    /// * ...
    /// * LFSR[95] = msb of IV[12]
    pub fn ivsetup(&mut self, iv: &[u8]) {
        for i in 0..(self.ivsize / 8) {
            for j in 0..8 {
                self.NFSR[i * 8 + j] = (self.key[i] >> j) & 1;
                self.LFSR[i * 8 + j] = (iv[i] >> j) & 1;
            }
        }

        for i in self.ivsize / 8..self.keysize / 8 {
            for j in 0..8 {
                self.NFSR[i * 8 + j] = (self.key[i] >> j) & 1;
                self.LFSR[i * 8 + j] = 1;
            }
        }

        /* do initial clockings */
        for _ in 0..256 {
            let outbit = self.keystream();
            self.LFSR[127] ^= outbit;
            self.NFSR[127] ^= outbit;
        }
    }

    /// Generates a new bit and updates the internal state of the cipher.
    fn keystream(&mut self) -> u8 {
        /* Calculate feedback and output bits */
        let outbit = self.NFSR[2]
            ^ self.NFSR[15]
            ^ self.NFSR[36]
            ^ self.NFSR[45]
            ^ self.NFSR[64]
            ^ self.NFSR[73]
            ^ self.NFSR[89]
            ^ self.LFSR[93]
            ^ (self.NFSR[12] & self.LFSR[8])
            ^ (self.LFSR[13] & self.LFSR[20])
            ^ (self.NFSR[95] & self.LFSR[42])
            ^ (self.LFSR[60] & self.LFSR[79])
            ^ (self.NFSR[12] & self.NFSR[95] & self.LFSR[95]);

        let n_bit = self.LFSR[0]
            ^ self.NFSR[0]
            ^ self.NFSR[26]
            ^ self.NFSR[56]
            ^ self.NFSR[91]
            ^ self.NFSR[96]
            ^ (self.NFSR[3] & self.NFSR[67])
            ^ (self.NFSR[11] & self.NFSR[13])
            ^ (self.NFSR[17] & self.NFSR[18])
            ^ (self.NFSR[27] & self.NFSR[59])
            ^ (self.NFSR[40] & self.NFSR[48])
            ^ (self.NFSR[61] & self.NFSR[65])
            ^ (self.NFSR[68] & self.NFSR[84]);

        let l_bit = self.LFSR[0]
            ^ self.LFSR[7]
            ^ self.LFSR[38]
            ^ self.LFSR[70]
            ^ self.LFSR[81]
            ^ self.LFSR[96];

        /* Update registers */
        for i in 1..self.keysize {
            self.NFSR[i - 1] = self.NFSR[i];
            self.LFSR[i - 1] = self.LFSR[i];
        }

        self.NFSR[(self.keysize) - 1] = n_bit;
        self.LFSR[(self.keysize) - 1] = l_bit;
        return outbit;
    }

    /// Generate keystream in bytes
    ///
    /// Assumptions
    /// * Bits are generated in order z0, z1, z2...
    ///
    /// The bits are stored in a byte in order:
    /// * lsb of keystream[0] = z0
    /// * ...
    /// * msb of keystream[0] = z7
    /// * ...
    /// * lsb of keystream[1] = z8
    /// * ...
    /// * msb of keystream[1] = z15
    /// * ...
    /// * ...
    pub fn keystream_bytes(&mut self, keystream: &mut [u8]) {
        for i in 0..keystream.len() {
            keystream[i] = 0;

            for j in 0..8 {
                keystream[i] |= self.keystream() << j;
            }
        }
    }

    pub fn encrypt_bytes(&mut self, ciphertext: &mut [u8]) {
        for i in 0..ciphertext.len() {
            let mut k = 0;

            for j in 0..8 {
                k |= self.keystream() << j;
            }
            ciphertext[i] = ciphertext[i] ^ k;
        }
    }

    pub fn decrypt_bytes(&mut self, plaintext: &mut [u8]) {
        for i in 0..plaintext.len() {
            let mut k = 0;

            for j in 0..8 {
                k |= self.keystream() << j;
            }
            
            plaintext[i] = plaintext[i] ^ k;
        }
    }
}