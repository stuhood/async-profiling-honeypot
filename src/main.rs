use scrypt::scrypt;

async fn slow(log_n: u8) -> Vec<u8> {
    let params = scrypt::Params::new(log_n, 8, 1).unwrap();
    let salt = [0, 1, 2, 3];
    let input = [0u8; 128];
    let mut output = input.clone();
    scrypt(&input, &salt, &params, &mut output).unwrap();
    output.to_vec()
}

#[tokio::main]
async fn main() {
    let log_n: u8 = std::env::args()
        .nth(1)
        .expect("Expected one integer argument.")
        .parse()
        .unwrap();
    let slow2 = tokio::spawn(slow(log_n));
    println!("slow1: {:?}", slow(log_n).await);
    println!("slow2: {:?}", slow2.await);
}
