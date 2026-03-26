pub fn modp(x: i128, p: i128) -> i128 {
    ((x % p) + p) % p
}

pub fn add_mod(a: i128, b: i128, p: i128) -> i128 {
    modp(a + b, p)
}

pub fn sub_mod(a: i128, b: i128, p: i128) -> i128 {
    modp(a - b, p)
}

pub fn mul_mod(a: i128, b: i128, p: i128) -> i128 {
    modp(a * b, p)
}

pub fn pow_mod(mut base: i128, mut exp: u128, p: i128) -> i128 {
    let mut result = 1i128;
    base = modp(base, p);

    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, p);
        }
        base = mul_mod(base, base, p);
        exp >>= 1;
    }

    result
}

pub fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (g, x1, y1) = extended_gcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (g, x, y)
    }
}

pub fn inv_mod(a: i128, p: i128) -> Result<i128, String> {
    let a = modp(a, p);
    if a == 0 {
        return Err("0 has no inverse modulo p".to_string());
    }

    let (g, x, _) = extended_gcd(a, p);
    if g != 1 {
        return Err(format!("{} has no inverse modulo {}", a, p));
    }

    Ok(modp(x, p))
}