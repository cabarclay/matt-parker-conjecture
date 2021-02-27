use rug::Float;

use std::fs::File;
use std::io::prelude::Read;
use std::time::SystemTime;

fn main() {
    // load digits of pi
    let mut f = File::open("pi.10000").expect("file failed to open");

    let mut digits_of_pi = String::new();
    f.read_to_string(&mut digits_of_pi).expect("could not load digits");

    digits_of_pi = digits_of_pi.trim_end().to_owned();

    //let mut pi_prog = String::from("3.");
    //let mut i_pi = digits_of_pi.chars();

    const Precision: u32 = 100_000_000;

    let mut d = String::from("3.");
    d.push_str(&digits_of_pi);

    let v = Float::parse(&d);
    let f_pi = Float::with_val(Precision, v.unwrap());

    pow_pi(Float::with_val(Precision, &f_pi), &f_pi, 10, Precision);
}

fn pow_pi(inp: Float, pi: &Float, depth: usize, precision: u32) -> Float {
    dbg!(&depth);

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => dbg!(n.as_secs()),
        Err(_) => panic!(""),
    };

    if depth == 0 {
        return inp;
    }

    let mut ln = Float::with_val(precision, inp.ln());
    ln = ln * pi;
    ln.exp_mut();

    println!("pi{}={}", depth, ln.to_string_radix(10, Some(1039_usize)));

    return pow_pi(ln, &pi, depth - 1, precision);
}
