use std::ops::Add;

pub struct StepIterator<T> {
    beg: T,
    end: T,
    step: T,
}

impl<T> StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg <= self.end {
            let res = self.beg;
            self.beg = self.beg + self.step;
            if res <= self.end {
                Some(res)
            } else {
                None
            }
        } else {
            None
        }
    }
}
