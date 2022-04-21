#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome(Vec<(u64, u64)>);

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome(vec![(a, b)])
    }

    pub fn value(&self) -> u64 {
        unimplemented!("return the value of this palindrome")
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.0.push((a, b));
    }
}

impl PartialOrd for Palindrome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Palindrome {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut products = (min..=max)
        .flat_map(|a| (min..=max).map(move |b| (a, b)))
        .filter_map(|(a, b)| {
            let digits = to_digits(a * b);
            if digits.iter().map(|x| x.clone()).rev().collect::<Vec<u64>>() == digits {
                Some(Palindrome::new(a, b))
            } else {
                None
            }
        })
        .collect::<Vec<Palindrome>>();

    dbg!(&products);
    products.sort();
    dbg!(&products);

    let mut pi = products.into_iter();

    Some((pi.nth(0).unwrap(), pi.last().unwrap()))
}

fn to_digits(num: u64) -> Vec<u64> {
    let mut vec = Vec::new();
    let mut rem = num;

    while rem > 0 {
        vec.push(rem % 10);
        rem = rem / 10;
    }

    return vec;
}
