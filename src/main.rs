use code_challenge::vigenere::decrypt;

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JRKUC
    ";
    let plaintext = decrypt(&ciphertext, key);

    println!("{}", plaintext);
}
