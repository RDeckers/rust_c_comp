#include <stdint.h>
#include <stdlib.h>

uint32_t usqrt(uint32_t x){ //fails on x= UINT_MAX, gives 0.
  uint32_t cur = x, next = (x+1)/2;
  while(next-cur > 1){
    cur = next;
    next = (cur + x/cur)/2;
  }
  return cur;
}

uint32_t* generate_primes_c_unsafe(uint32_t *restrict primes, uint32_t max_n_primes){
  if(!primes){
    return NULL;
  }
  const uint32_t seed_primes[] = {2, 3, 5 , 7, 11, 13, 17, 19, 23, 29};
  const size_t n_seed_primes = sizeof(seed_primes)/sizeof(seed_primes[0]);
  if(max_n_primes < n_seed_primes){
    for(unsigned u = 0; u < max_n_primes; u++){
      primes[u] = seed_primes[u];
    }
    return primes;
  }
  for(unsigned u = 0; u < n_seed_primes; u++){
    primes[u] = seed_primes[u];
  }
  size_t prime_count = n_seed_primes;
  uint32_t step = 2;
  uint32_t test_prime = primes[prime_count-1];
  while(prime_count < max_n_primes){
    TEST_NEXT_PRIME:
    test_prime += step;
    step ^= 6;
    uint32_t limit = usqrt(test_prime)+1;
    for(unsigned u = 2; primes[u] < limit; u++){
      if(0 == test_prime % primes[u]){
        goto TEST_NEXT_PRIME;
      }
    }
    primes[prime_count++] = test_prime;
  }
  return primes;
}
