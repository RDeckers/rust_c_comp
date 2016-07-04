#![feature(test)]
#![feature(core_intrinsics)]
#![feature(box_syntax)]
#[cfg(test)]
extern crate test;
extern crate core;

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt_for{
    ($T:ty) => (
        impl Sqrt for $T{
            #[inline]
            fn sqrt(self) -> Self{
                let mut cur  = self;
                let mut next = (cur+1)/2;
                while next.wrapping_sub(cur) > 1 {
                    cur = next;
                    next = (cur + self/cur)/2;
                }
                next
            }
        }
    )
}

impl_sqrt_for!(u32);


extern "C"{
    pub fn generate_primes_c_unsafe(buffer : *mut u32, up_to: u32);//Probably better to have rust do the allocation and pass the buffer as an arg.
}

pub fn generate_primes_c(up_to: usize) -> Vec<u32>{
    let mut prime_slice = Vec::with_capacity(up_to);
    unsafe{
        generate_primes_c_unsafe(prime_slice.as_mut_ptr(), up_to as u32);
        prime_slice.set_len(up_to);
    }
    prime_slice
}

#[no_mangle]
pub fn generate_primes_rs(up_to: usize) -> Vec<u32>{
    use core::intrinsics::unchecked_rem;
    let mut prime_slice = Vec::with_capacity(up_to);
    prime_slice.extend([2, 3, 5 , 7, 11, 13, 17, 19, 23, 29].iter().cloned());
    let mut step : u32 = 2;
    let mut prime_count = prime_slice.len();
    let mut test_prime = prime_slice[prime_count-1];
    unsafe{prime_slice.set_len(up_to);}
    while prime_count < up_to{
        test_prime += step;
        step ^= 6;
        let mut limit = test_prime.sqrt()+1;
         while prime_slice[2..].iter().take_while(|&p| *p < limit).any(|&p| unsafe{unchecked_rem(test_prime, p)} == 0){
            test_prime += step;
            step ^= 6;
            limit = test_prime.sqrt()+1;
        }
        unsafe{*prime_slice.get_unchecked_mut(prime_count) = test_prime;}
        prime_count += 1;
    }
    prime_slice
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_10k_c() {
        let primes = generate_primes_c(10_000);
        println!("{:?}", &primes[0..30]);
        assert_eq!(primes[10_000-1], 104729);
    }
    #[test]
    fn verify_10k_rust() {
        let primes = generate_primes_rs(10_000);
        println!("{:?}", &primes[0..30]);
        assert_eq!(primes[10_000-1], 104729);
    }
    use super::*;
    use test::Bencher;
    use test::black_box;
    #[bench]
    fn rust(b: &mut Bencher) {
        b.iter(||{
            let n = black_box(15_000);
            generate_primes_rs(n)
        });
    }

    #[bench]
    fn c(b: &mut Bencher) {
        b.iter(||{
            let n = black_box(15_000);
            generate_primes_c(n)
        });
    }
}
