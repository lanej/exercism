#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    set: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: Clone + PartialOrd,
{
    pub fn new(input: &[T]) -> Self {
        Self {
            set: input.to_vec(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(element)
    }

    pub fn add(&mut self, _element: T) {
        unimplemented!();
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.set.is_empty() {
            return true;
        }

        self.set.iter().fold(!other.is_empty(), |is_subset, e| {
            is_subset & other.set.contains(e)
        })
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}
