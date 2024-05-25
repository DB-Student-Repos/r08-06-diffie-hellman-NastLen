use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();

    let private_key: u64 = rng.gen_range(2..p);
    private_key
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 { // p is prime number, g is generator, a is private key
    // we can use MODULAR EXPONENTIATION, which is a method of calculating a^b mod p without causing overflow
    let mut public_key = 1;
    let mut base = g % p;
    let mut exp = a;

    while exp > 0 {
        if exp % 2 == 1 {
            public_key = (public_key * base) % p;
        }
        base = (base * base) % p;
        exp /= 2;
    }

    public_key
    
    // unimplemented!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let mut res = 1;
    let mut base = b_pub % p;
    let mut exp = a;

    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % p;
        }
        base = (base * base) % p;
        exp /= 2;
    }

    res
    // unimplemented!(
    //     "Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}"
    // )
}
