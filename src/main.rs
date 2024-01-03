const OFFSET: u8 = 64;

fn encrypt(str: String, key: &String) -> String {
    let key_bytes = key.as_bytes();
    let str_bytes = str.as_bytes();
    
    let mut i = 0;

    let mut new_bytes:Vec<u8> = Vec::new();
    for byte in str_bytes {
        let mut new_byte = byte.wrapping_sub(key_bytes[i]);
        new_byte = new_byte.wrapping_add(OFFSET);
        new_bytes.push(new_byte);

        if i+1 >= key_bytes.len() {
            i = 0;
        } else {
            i += 1;
        }
    }

    return String::from_utf8(new_bytes).expect("idiot");
}

fn decrypt(encrypted: String, key: &String) -> String {
    let enc_bytes = encrypted.as_bytes();
    let key_bytes = key.as_bytes();

    let mut i = 0;

    let mut new_bytes:Vec<u8> = Vec::new();
    for byte in enc_bytes {
        let mut new_byte = byte.wrapping_sub(OFFSET);
        new_byte = new_byte.wrapping_add(key_bytes[i]);
        new_bytes.push(new_byte);

        if i+1 >= key_bytes.len() {
            i = 0;
        } else {
            i += 1;
        }
    }

    return String::from_utf8(new_bytes).expect("idiot");
}

fn main() {
    let str = String::from("meow");
    let key = String::from("jmac");

    let encrypted = encrypt(str, &key);

    println!("{}", encrypted);

    let decrypted = decrypt(encrypted, &key);
    println!("{}", decrypted);
}
