pub fn nth(n: u32) -> u32 {
    let is_prime = |num: u32| {
        let max_factor = (num as f64).sqrt() as u32;
        let mut counter: u32 = 2;

        while counter <= max_factor {
            if num % counter == 0 {
                return false
            }
            counter = counter + 1
        }

        return true
    };

    let mut primes: Vec<u32> = vec![];
    let mut candidate: u32 = 2;

    while primes.len() <= n as usize {
        if is_prime(candidate) {
            primes.push(candidate)
        }
        candidate = candidate + 1
    }

    return *primes.last().unwrap()
}
