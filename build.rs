extern crate gcc;

fn main() {
    gcc::compile_library("libprime.a", &["src/primes.c"]);
}
