pub struct Memory<const T: usize> {
    /// Inner representation of memory
    pub inner: [u8; T],
    /// Data pointer
    pub dp: usize,
}

impl<const T: usize> Memory<T> {
    pub fn new() -> Self {
        Self {
            inner: [0u8; T],
            dp: 0,
        }
    }

    /// adds 1 to the current data pointer
    pub fn add(&mut self) {
        let val = self.inner[self.dp].wrapping_add(1);
        self.inner[self.dp] = val;
    }

    /// substracts 1 to the current data pointer
    pub fn sub(&mut self) {
        let val = self.inner[self.dp].wrapping_sub(1);
        self.inner[self.dp] = val;
    }

    pub fn next(&mut self) {
        let val = self.dp + 1;
        if val > T {
            self.dp = 0;
        } else {
            self.dp = val;
        }
    }

    pub fn prev(&mut self) {
        self.dp = self.dp.checked_sub(1).unwrap_or(T);
    }

    pub fn should_loop(&self) -> bool {
        self.inner[self.dp] != 0
    }

    pub fn get(&self) -> u8 {
        self.inner[self.dp]
    }
}
