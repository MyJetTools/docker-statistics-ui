use std::collections::VecDeque;

pub struct HistoryCache<T: Clone> {
    data: VecDeque<T>,
    max_amount: usize,
}

impl<T: Clone> HistoryCache<T> {
    pub fn new(max_amount: usize) -> Self {
        Self {
            data: VecDeque::new(),
            max_amount,
        }
    }

    pub fn add(&mut self, value: T) {
        while self.data.len() >= self.max_amount {
            self.data.pop_front();
        }

        self.data.push_back(value);
    }

    pub fn get_snapshot(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.data.len());

        for itm in self.data.iter() {
            result.push(itm.clone());
        }

        result
    }
}
