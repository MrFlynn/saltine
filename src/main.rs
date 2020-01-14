extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

static BASE64_CRYPT: &'static str = &"./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
static BASE64_INDICES: &'static [usize] = &[
    0, 6, 12, 1, 7, 13, 2, 8, 14, 3, 9, 15, 4, 10, 5, 11
];

fn create_intermediate_hash(password: &String, salt: &String) -> [u8; 16] {
    let mut intermediate = Md5::new();
    let mut alternate = Md5::new();

    // Push initial values on to intermediate string.
    intermediate.input_str(password);
    intermediate.input_str("$1$");
    intermediate.input_str(salt);

    // Generate alternate hash from password and salt.
    alternate.input_str(&format!("{p}{s}{p}", p=password, s=salt));
    
    // Copy result of hash to vector of u8.
    let mut alternate_sum = [0; 16];
    alternate.result(&mut alternate_sum);
    
    // Push n bytes of alternate on to intermediate.
    for i in 0..password.len() {
        intermediate.input(&[alternate_sum[i]]);
    }
    alternate.reset();

    // For each bit in the password length, append either a null byte or the first character of 
    // the password on to the intermediate.
    let first_char = password.as_str().chars().next().unwrap().to_string();

    let mut bit_position = password.len();
    while bit_position != 0 {
        if (bit_position & 1) == 1 {
            intermediate.input(&[0]);
        } else {
            intermediate.input_str(&first_char);
        }

        bit_position >>= 1;
    }

    // Copy result to output buffer.
    let mut buf = [0; 16];
    intermediate.result(&mut buf);

    buf
}

fn compute_password_hash(password: &String, salt: &String, intermediate: [u8; 16]) -> Vec<u8> {
    let mut new_intermediate: [u8; 16] = [0; 16];
    new_intermediate[..16].copy_from_slice(&intermediate);

    let mut hasher = Md5::new();
    
    for i in 0..1000 {
        if (i & 1) != 0 {
            hasher.input_str(password);
        } else {
            hasher.input(&new_intermediate);
        }

        if (i % 3) != 0 {
            hasher.input_str(salt);
        }

        if (i % 7) != 0 {
            hasher.input_str(password);
        }

        if (i & 1) != 0 {
            hasher.input(&new_intermediate);
        } else {
            hasher.input_str(password);
        }

        // Copy result back to new_intermediate.
        hasher.result(&mut new_intermediate);
        hasher.reset();
    }

    new_intermediate.to_vec()
}

fn packed_base64_encode(input_string: &mut String, mut packed_value: u32, mut size: u8) {
    while size > 0 {
        size -= 1;

        let c = (packed_value & 0x3f) as usize;
        input_string.push(BASE64_CRYPT.as_bytes()[c] as char);

        packed_value >>= 6;
    }
}

fn crypt_base64_encode(hash: Vec<u8>) -> String {
    let mut encoded = String::new();

    for indices in BASE64_INDICES.chunks(3) {
        if indices.len() == 3 {
            let val = (hash[indices[0]] as u32) << 16 | (hash[indices[1]] as u32) << 8 | hash[indices[2]] as u32;
            packed_base64_encode(&mut encoded, val, 4);
        } else if indices.len() == 1 {
            let val = hash[indices[0]] as u32;
            packed_base64_encode(&mut encoded, val, 2);
        }
    }

    encoded
}

fn main() {
    let password = String::from("thisisatest");
    let salt = String::from("hello");

    let intermediate = create_intermediate_hash(&password, &salt);
    let password_hash = compute_password_hash(&password, &salt, intermediate);

    println!("{:?}", crypt_base64_encode(password_hash));
}
