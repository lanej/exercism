#[derive(Debug)]
pub struct CircularBuffer<T: Clone + Default> {
    buffer: Vec<T>,
    size: usize,
    oldest: usize,
    latest: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            buffer.push(T::default());
        }

        Self {
            buffer,
            oldest: 0,
            size: 0,
            latest: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.size == self.buffer.capacity() {
            return Err(Error::FullBuffer);
        }

        let next = self.next(self.latest);
        self.buffer[next] = element;
        self.size += 1;
        self.latest = next;

        return Ok(());
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.size == 0 {
            return Err(Error::EmptyBuffer);
        }

        let next = self.next(self.oldest);
        let value = &self.buffer[self.oldest];
        self.size -= 1;
        self.oldest = next;

        return Ok(value.clone());
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.oldest = 0;
        self.size = 0;
        self.latest = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buffer.len() <= self.buffer.capacity() {
            self.write(element).unwrap();
            return;
        }

        self.buffer[self.oldest] = element;
        self.oldest = self.next(self.oldest);
    }

    fn next(&self, ptr: usize) -> usize {
        if self.size == 0 {
            return ptr;
        }
        return (ptr + 1) % self.buffer.capacity();
    }
}
