# cryptool
openssl example in rust.

Currently, only AES-128-CBC encryption and decryption is implemented.

## example
```bash
./cryptool --help
cryptool 0.1.0
My own crypto tool in Rust.

USAGE:
    cryptool [FLAGS] [OPTIONS] -c <cipher> -d -e -V <iv> -K <key>

FLAGS:
    -d               Decryption
    -e               Encryption
    -h, --help       Prints help information
        --version    Prints version information

OPTIONS:
    -c <cipher>              Specify the cipher, support: aes-128-cbc
    -i <input-string>        Input base64 encoded data
    -V <iv>                  IV to use, specified as a hexadecimal string, must be 16 byte. eg:
                             33333333333333333333333333333333
    -K <key>                 Key to use, specified as a hexadecimal string, must be 16 byte. eg:
                             33333333333333333333333333333333
```

```
 ~/cryptool/target/release/ [master] echo -n "Rust -- A language empowering everyone to build reliable and efficient software." | base64 | xargs ./cryptool -c aes-128-cbc -K 3171617a327773783365646334726676 -V 3679686e357467623679686e3679686e -e -i
Output:: KkEpNSXiNMC7WuLxQ7I8j5mOm7XfozBoYMWDtBGWhcPuQxJNz443Brdhz6h2PlNQj+yCTdkB+oprJ+iE444vDZz7mNLkSrJeA7owsQ+cRt3E4rsQXaYSsH5NJyEqgAp2
 ~/cryptool/target/release/ [master]
 ~/cryptool/target/release/ [master] ./cryptool -c aes-128-cbc -K 3171617a327773783365646334726676 -V 3679686e357467623679686e3679686e -d -i KkEpNSXiNMC7WuLxQ7I8j5mOm7XfozBoYMWDtBGWhcPuQxJNz443Brdhz6h2PlNQj+yCTdkB+oprJ+iE444vDZz7mNLkSrJeA7owsQ+cRt3E4rsQXaYSsH5NJyEqgAp2

<output in base64>
UnVzdCAtLSBBIGxhbmd1YWdlIGVtcG93ZXJpbmcgZXZlcnlvbmUgdG8gYnVpbGQgcmVsaWFibGUgYW5kIGVmZmljaWVudCBzb2Z0d2FyZS4=

<output in plain>
Rust -- A language empowering everyone to build reliable and efficient software.
```

