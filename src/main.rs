use rand_distr::{Distribution, Normal};
mod programs;

#[derive(Debug, Default)]
struct RunStats {
    hits: u32,
    faults: u32,
    hit_ratio: f64,
    fault_ratio: f64,
}

impl RunStats {
    fn calc_hit_ratio(&mut self) {
        self.hit_ratio = self.hits as f64 / (self.faults + self.hits) as f64
    }
    fn calc_fault_ratio(&mut self) {
        self.fault_ratio = self.faults as f64 / (self.faults + self.hits) as f64
    }
    fn calc_ratios(&mut self) {
        self.calc_hit_ratio();
        self.calc_fault_ratio();
    }
}
struct TestRun {
    pageref_sequence: Vec<u32>,
    n_frames: usize,
    stats: RunStats,
    hit_fault_sequence: Vec<bool>,
    frame_sequence: Vec<Vec<Option<u32>>>,
}

impl TestRun {
    fn new(pageref_sequence: &[u32], n_frames: usize) -> Self {
        Self {
            pageref_sequence: pageref_sequence.to_vec(),
            n_frames,
            stats: Default::default(),
            hit_fault_sequence: Vec::new(),
            frame_sequence: vec![vec![None; n_frames]],
        }
    } 

    fn FIFO(&mut self){
        for (i, pageref) in self.pageref_sequence.clone().into_iter().enumerate() {
            let frames = &self.frame_sequence[i];
            let mut new_frames = frames.clone();
            if frames.contains(&Some(pageref)) {
                self.hit_fault_sequence.push(true);
                self.stats.hits += 1;
            } else {
                self.hit_fault_sequence.push(false);
                self.stats.faults += 1;
                if let Some(frame) = new_frames.iter_mut().rev().take_while(|page| **page == None).last() {
                    *frame = Some(pageref);
                } else {
                    for f in self.frame_sequence.iter().rev().skip(1) {
                        let mut frame_index = None;
                        for (i,p) in f.into_iter().filter(|p| **p != None).enumerate() {
                            if p.unwrap() == new_frames[i].unwrap() {
                                if frame_index == None {
                                    frame_index = Some(i);
                                } else {
                                    frame_index = None;
                                    break;
                                }
                            }
                        }
                        if let Some(i) = frame_index {
                            new_frames[i] = Some(pageref);
                        }
                    }
                }
            }
            self.frame_sequence.push(new_frames);
        }
        self.stats.calc_ratios();
    }

    fn LRU(&mut self){
        for (i, pageref) in self.pageref_sequence.clone().into_iter().enumerate() {
            let frames = &self.frame_sequence[i];
            let mut new_frames = frames.clone();
            if frames.contains(&Some(pageref)) {
                self.hit_fault_sequence.push(true);
                self.stats.hits += 1;
            } else {
                self.hit_fault_sequence.push(false);
                self.stats.faults += 1;
                if let Some(frame) = new_frames.iter_mut().rev().take_while(|page| **page == None).last() {
                    *frame = Some(pageref);
                } else {
                    let mut used = vec![false; self.n_frames];
                    for u_index in (0..i).rev() {
                        if let Some(f) = new_frames.iter().position(|&p| p.unwrap() == self.pageref_sequence[u_index]) {
                            if used.iter().filter(|&f| *f == false).count() == 1 {
                                new_frames[f] = Some(pageref);
                                break;
                            }
                            used[f] = true;
                        }
                    }
                }
            }
            self.frame_sequence.push(new_frames);
        }
        self.stats.calc_ratios();
    }

    fn Optimal(&mut self){
        for (i, pageref) in self.pageref_sequence.clone().into_iter().enumerate() {
            let frames = &self.frame_sequence[i];
            let mut new_frames = frames.clone();
            if frames.contains(&Some(pageref)) {
                self.hit_fault_sequence.push(true);
                self.stats.hits += 1;
            } else {
                self.hit_fault_sequence.push(false);
                self.stats.faults += 1;
                if let Some(frame) = new_frames.iter_mut().rev().take_while(|page| **page == None).last() {
                    *frame = Some(pageref);
                } else {
                    let mut used = vec![false; self.n_frames];
                    for u_index in i+1..self.pageref_sequence.len() {
                        if let Some(f) = new_frames.iter().position(|&p| p.unwrap() == self.pageref_sequence[u_index]) {
                            if used.iter().filter(|&f| *f == false).count() == 1 {
                                new_frames[f] = Some(pageref);
                                break;
                            }
                            used[f] = true;
                        }
                        if u_index == self.pageref_sequence.len() - 1 {
                            new_frames[used.iter().position(|&p| p == false).unwrap()] = Some(pageref);
                        }
                    }
                }
            }
            self.frame_sequence.push(new_frames);
        }
        self.stats.calc_ratios();
    }
}

fn main(){
    let prog = programs::Program::new(50, 5000);
    let refs: Vec<u32> = prog.pageref_sequence.collect();
    let mut fifo_test = TestRun::new(&refs, 10);
    fifo_test.FIFO();
    let mut lru_test = TestRun::new(&refs, 10);
    lru_test.LRU();
    let mut optimal_test = TestRun::new(&refs, 10);
    optimal_test.Optimal();
    println!("fifo stats:\n{:#?}", fifo_test.stats);
    println!("LRU stats:\n{:#?}", lru_test.stats);
    println!("Optimal stats:\n{:#?}", optimal_test.stats);
}
