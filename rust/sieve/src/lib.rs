pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut seive: Vec<Option<u64>> = Vec::new();
    let mut initial: u64 = 1;
    seive.resize_with((upper_bound - 1) as usize, || {
        initial += 1;
        Some(initial)
    });

    let answers: Vec<u64> = Vec::new();

    for candidate in seive {
        match candidate {
            None => continue,
            Some(multiple) => {
                while multiple < (upper_bound as u64) {
                    seive.as_ref().insert((multiple - 2) as usize, None);
                }
            }
        }
    }

    return answers;
}
