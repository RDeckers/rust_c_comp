#![feature(test)]
#![feature(core_intrinsics)]
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
                use core::intrinsics::unchecked_div;
                let mut cur  = self;
                let mut next = (cur+1)/2;
                while next.wrapping_sub(cur) > 1 {
                    cur = next;
                    next = (cur + unsafe{unchecked_div(self,cur)})/2; //TODO: is this div always safe?
                }
                next
            }
        }
    )
}

impl_sqrt_for!(u32);


extern "C"{
    pub fn generate_primes_c_unsafe(buffer : *mut u32, up_to: u32);
}

#[no_mangle]
pub fn fill_primes_rs(primes : &mut [u32]){
    use core::intrinsics::unchecked_rem;
    let prime_capacity = primes.len();
    let seed_primes = [2, 3, 5 , 7, 11, 13, 17, 19, 23, 29];
    let n_seed_primes = seed_primes.len();
    if prime_capacity < n_seed_primes{
        for u in 0..prime_capacity{
            unsafe{*primes.get_unchecked_mut(u) = *seed_primes.get_unchecked(u)};
        }
    }else{
        for u in 0..n_seed_primes{
            unsafe{*primes.get_unchecked_mut(u) = *seed_primes.get_unchecked(u)};
        }
        let mut prime_count = n_seed_primes;


        let mut test_prime = *unsafe{primes.get_unchecked(prime_count-1)}+2; //31
        let mut step = 4;
        let mut limit = 6;//isqrt(31)+1 = 5 +1 = 6
        let mut steps_till_next_root = 5; //37 is the first x for which ceil(root(x)) > ceil(sqrt(31))
        let mut steps_made = 0;
        let mut u = 2;
        while prime_count < prime_capacity {
            while *unsafe{primes.get_unchecked(u)} < limit{
                if 0 == unsafe{unchecked_rem(test_prime, *primes.get_unchecked(u))}{
                    test_prime += step;
                    steps_made += step;
                    if steps_made >= steps_till_next_root{
                        steps_made -= steps_till_next_root;
                        steps_till_next_root = limit*2+1;
                        limit += 1;
                    }
                    step ^= 6;
                    u = 2;
                    continue;
                }
                u+=1;
            }
            unsafe{*primes.get_unchecked_mut(prime_count) = test_prime;}
            prime_count+=1;
            test_prime += step;
            steps_made += step;
            if steps_made >= steps_till_next_root{
                steps_made -= steps_till_next_root;
                steps_till_next_root = limit*2+1;
                limit += 1;
            }
            step ^= 6;
            u = 2;
        }
    }
}

pub fn generate_primes_rs(up_to: usize) -> Vec<u32>{
    let mut prime_slice : Vec<u32> = Vec::with_capacity(up_to);
    unsafe{prime_slice.set_len(up_to);}
    fill_primes_rs(prime_slice.as_mut_slice());
    prime_slice
}

#[no_mangle]
pub fn generate_primes_c(up_to: usize) -> Vec<u32>{
    let mut prime_slice = Vec::with_capacity(up_to);
    unsafe{
        generate_primes_c_unsafe(prime_slice.as_mut_ptr(), up_to as u32);
        prime_slice.set_len(up_to);
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
            let n = black_box(50_000);
            generate_primes_rs(n)
        });
    }

    #[bench]
    fn c(b: &mut Bencher) {
        b.iter(||{
            let n = black_box(50_000);
            generate_primes_c(n)
        });
    }
}
