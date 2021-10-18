use base64::{decode, encode};
use hex;
use openssl::symm::{decrypt, encrypt, Cipher};
use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::{collections::HashMap, io::BufRead};
use structopt::StructOpt;

// #[derive(Debug)]
struct cipherConf {
    cipherType: Cipher,
    key: [u8; 16],
    iv: [u8; 16],
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cryptool", about = "My own crypto tool in Rust.")]
pub struct Opt {
    /// Specify the cipher, support: aes-128-cbc
    ///
    #[structopt(short, required(true))]
    pub cipher: String,

    /// Key to use, specified as a hexadecimal string, must be 16 byte.
    /// eg: 33333333333333333333333333333333
    #[structopt(short = "K")]
    pub key: String,

    /// IV to use, specified as a hexadecimal string, must be 16 byte.
    /// eg: 33333333333333333333333333333333
    #[structopt(short = "V")]
    pub iv: String,

    /// Encryption
    #[structopt(short="e",conflicts_with_all(&["decrypt"]),required_unless("decrypt"))]
    pub encrypt: bool,

    /// Decryption
    #[structopt(short = "d",conflicts_with_all(&["encrypt"]),required_unless("encrypt"))]
    pub decrypt: bool,

    /// Input base64 encoded data
    #[structopt(short = "i")]
    pub inputString: Option<String>,
}
fn main() {
    let mut cipherconf = cipherConf {
        cipherType: Cipher::aes_128_cbc(),
        key: [0; 16],
        iv: [0; 16],
    };
    let opt = Opt::from_args();
    // println!("{:#?}", opt);

    let availableCipher: HashMap<String, Cipher> =
        [("aes-128-cbc".to_owned(), Cipher::aes_128_cbc())]
            .iter()
            .cloned()
            .collect();

    // let availableCipher = vec!["aes-128-cbc".to_owned()];

    // if !availableCipher.contains(&opt.cipher) {
    //     panic!("Bad Cipher Type!");
    // }
    // cipherconf.cipherType = opt.cipher.clone();

    match availableCipher.get(&opt.cipher) {
        Some(cipher) => {
            cipherconf.cipherType = cipher.clone();
        }
        _ => panic!("Bad Cipher Type!"),
    }

    let decoded_tmp = hex::decode(opt.key);
    match decoded_tmp {
        Ok(aaa) => match aaa.try_into() {
            Ok(key) => cipherconf.key = key,
            Err(_) => {
                panic!("The key failed to decode to 16 bytes array.")
            }
        },
        Err(_) => {
            panic!("The key is not valid hex.")
        }
    }

    let decoded_tmp = hex::decode(opt.iv);
    match decoded_tmp {
        Ok(aaa) => match aaa.try_into() {
            Ok(iv) => cipherconf.iv = iv,
            Err(_) => {
                panic!("The iv failed to decode to 16 bytes array.")
            }
        },
        Err(_) => {
            panic!("The iv is not valid hex.")
        }
    }
    // for qqq in &availableCipher {
    //     println!("{}",qqq);
    // }

    if let Some(inputdata) = opt.inputString {
        // println!("input string: {}", inputdata);
        let mut indata = Vec::new();
        if let Ok(rawdata) = decode(&inputdata) {
            indata = rawdata;
        } else {
            panic!("The input data is not valid base64 string.");
        }
        // println!("decoded: {}",String::from_utf8_lossy(&indata));

        if opt.encrypt {
            let ciphertext = match encrypt(
                cipherconf.cipherType,
                &cipherconf.key,
                Some(&cipherconf.iv),
                // inputdata.as_bytes(),
                indata.as_ref(),
            ) {
                Ok(cipherdata) => cipherdata,
                Err(why) => {
                    println!("{:?}", why);
                    vec![]
                }
            };
            println!("Output:: {}",encode(&ciphertext));
            // io::stdout().write_all(&ciphertext);    // STDOUT output.
        }

        if opt.decrypt{
            let ciphertext = match decrypt(
                cipherconf.cipherType,
                &cipherconf.key,
                Some(&cipherconf.iv),
                // inputdata.as_bytes(),
                indata.as_ref(),
            ) {
                Ok(cipherdata) => cipherdata,
                Err(why) => {
                    println!("{:?}", why);
                    vec![]
                }
            };
            println!("\n<output in base64>\n{}",encode(&ciphertext));
            println!("\n<output in plain>\n{}",String::from_utf8_lossy(ciphertext.as_ref()))
        }
    } else {
        println!("todo:: input data from STDIN");
        // let input = io::stdin().read(buf)
        // let mut input = String::new();
        // match io::stdin().read_line(&mut input) {
        //     Ok(n) => {
        //         println!("{} bytes read", n);
        //         println!("{}", input);
        //     }
        //     Err(error) => println!("error: {}", error),
        // }
    }
    // println!("{:?}", cipherconf);
}

#[cfg(test)]
mod test {
    use base64::{decode, encode};
    use openssl::{
        encrypt,
        symm::{decrypt, encrypt, Cipher},
    };

    #[test]
    fn cbctest() {
        let cipher = Cipher::aes_128_cbc();
        let data = b"Some Crypto Text0Text0Text0Text0Text0Text0Text0Text0";
        let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
        let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

        let key = b"KbXbWl7TZaxURxNA";
        let iv = b"KbXbWl7TZaxURxNA";
        let ciphertext = match encrypt(cipher, key, Some(iv), data) {
            Ok(cipherdata) => cipherdata,
            Err(why) => {
                println!("{:?}", why);
                vec![]
            }
        };

        println!(
            "plain data: {}\nencrypted data: {}",
            String::from_utf8_lossy(data),
            encode(ciphertext.clone())
        );

        // assert_eq!(
        //     b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\x87\x4D\
        //   \xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1",
        //     &ciphertext[..]
        // );

        // println!("{:?}",decode(encode(ciphertext.clone())).unwrap());
        let aaa = decrypt(
            cipher,
            key,
            Some(iv),
            decode(encode(ciphertext.clone())).unwrap().as_ref(),
        );
        // println!("decrypted data(base64): {}",encode(aaa.unwrap()));
        println!(
            "decrypted data: {}",
            String::from_utf8_lossy(aaa.unwrap().as_ref())
        );

        let ciphertext = match encrypt(cipher, key, Some(iv), b"") {
            Ok(cipherdata) => cipherdata,
            Err(why) => {
                println!("{:?}", why);
                vec![]
            }
        };

        println!("empty input, encrypted data: {}", encode(ciphertext));
    }
}
