use std::cell::Cell;

pub struct License {
    max_count: u32,
    count: Cell<u32>,
}

impl License {
    pub fn with_max_count(max_count: u32) -> Self {
        Self {
            max_count,
            count: Cell::new(0),
        }
    }

    pub fn protect(&self) -> Result<LicenseGuard, String> {
        let current_count = self.count.get();

        if current_count < self.max_count {
            self.count.set(current_count + 1);

            Ok(LicenseGuard { source: self })
        } else {
            Err("Avalilable licenses are exhausted".to_string())
        }
    }

    fn release(&self) {
        self.count.set(self.count.get() - 1);
    }
}

pub struct LicenseGuard<'a> {
    source: &'a License,
}

impl<'a> Drop for LicenseGuard<'a> {
    fn drop(&mut self) {
        self.source.release();
    }
}
