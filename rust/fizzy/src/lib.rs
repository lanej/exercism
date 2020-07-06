// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use core::ops::Rem;

pub type MatchFn<T> = Box<dyn Fn(T) -> bool>;

pub struct Matcher<T> {
    matcher: MatchFn<T>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: AsRef<str>,
    {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.as_ref().to_string(),
        }
    }
}

#[derive(Default)]
pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T> Fizzy<T>
where
    T: Copy + ToString,
{
    pub fn new() -> Self {
        Fizzy(Vec::new())
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        let Fizzy(ref mut matchers) = self;
        matchers.push(matcher);
        self
    }

    fn apply_to(&self, item: T) -> String {
        let Fizzy(ref matchers) = self;
        let mut out = String::new();
        for matcher in matchers {
            let matcherfn = &(matcher.matcher);

            if matcherfn(item) {
                out += &matcher.subs;
            }
        }
        if out.is_empty() {
            out = item.to_string()
        }
        out
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, _iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        _iter.map(move |item| self.apply_to(item))
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + Default + ToString + From<u8> + PartialEq + Rem<Output = T> + 'static,
{
    let three: T = 3.into();
    let five: T = 5.into();

    Fizzy(vec![
        Matcher::new(move |n| n % three == T::default(), "fizz"),
        Matcher::new(move |n| n % five == T::default(), "buzz"),
    ])
}
