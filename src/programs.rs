use rand_distr::{Distribution, Normal};

pub struct RefSeq {
    num_pages: u32,
    cur_page: u32,
    length: u32,
    last_locality_change: u32,
}

impl Iterator for RefSeq {
    type Item = u32;

    fn next(&mut self) -> Option::<Self::Item> {
        self.length -= 1;
        if self.length > 0 {
            let next_page = (Normal::new(self.cur_page as f32, ((self.last_locality_change - self.length) as f32).powf(2.0)).unwrap()
                .sample(&mut rand::thread_rng()) as u32)
                .clamp(0, self.num_pages);
            if self.cur_page.max(next_page) - self.cur_page.min(next_page) > 2 {
                self.last_locality_change = self.length;
            }
            self.cur_page = next_page;
            Some(self.cur_page)
        } else { None }
    }
}

impl RefSeq {
    pub fn new(length: u32, num_pages: u32) -> Self {
        Self { num_pages, cur_page: 0, length, last_locality_change: length }
    }
}

pub struct Program {
    page_table: Vec<u32>,
    last_page: u32,
    pub pageref_sequence: RefSeq,
}

impl Program {
    pub fn new(num_pages: u32, runtime: u32) -> Self {
        Self {
            page_table: Vec::<u32>::with_capacity(num_pages as usize),
            last_page: num_pages - 1,
            pageref_sequence: RefSeq::new(runtime, num_pages),
        }
    }
}
