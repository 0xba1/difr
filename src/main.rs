fn main() {
    println!(
        "{} : {}",
        tiny_keccak::sha3(b"hello"),
        tiny_keccak::sha3(b"hello")
    );
}
