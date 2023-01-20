extern crate rand;

use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}

pub fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
//    println!("Calculating {}^{} % {}", base, exp, modulus);
    fn recur(pow2: u64, pow_base: u64, exp: u64, modulus: u64, prod_mods: u64) -> u64 {
//        println!("pow2: {}, pow_base: {}, prod_mods: {}", pow2, pow_base, prod_mods);
        if pow2 > exp {
            prod_mods % modulus
        } else {
            recur(2*pow2, (pow_base * pow_base) % modulus, exp, modulus,
                if exp & pow2 == 0 { prod_mods } else {
                    prod_mods * pow_base
                }
            )
        }
    }
    recur(1, base % modulus, exp, modulus, 1)
}
