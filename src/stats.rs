use std::f32;
use std::fmt;

#[derive(Debug)]
pub struct Histogram {
    /// Upper bounds of bins.
    /// Lower bound of first bin is f32::MIN by definition.
    bounds: Vec<f32>,

    /// Counds for the respective bins.
    counts: Vec<i64>,
}

impl Histogram {
    pub fn new_from_bounds(mut bounds: Vec<f32>) -> Histogram {
        bounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
        bounds.dedup();
        bounds.push(f32::MAX);
        let counts = vec![0; bounds.len()];
        Histogram { bounds, counts }
    }

    pub fn new(min: f32, max: f32, n: i32) -> Histogram {
        let mut bounds = Vec::new();
        let step = (max - min) / (n - 1) as f32;
        for i in 0..n {
            bounds.push(min + (i as f32) * step);
        }
        return Histogram::new_from_bounds(bounds);
    }

    pub fn insert(&mut self, value: f32) {
        for (i, v) in self.bounds.iter().enumerate() {
            if *v > value {
                self.counts[i] += 1;
                return;
            }
        }
        panic!("Could not insert value {}.", value);
    }

    pub fn reset(&mut self) {
        self.counts = vec![0; self.bounds.len()];
    }

    pub fn plot_ascii(&self) {
        let max = *self.counts.iter().max().unwrap();
        let height = 50;
        let scale: f64 = height as f64 / max as f64;
        for i in (0..height).rev() {
            for &c in self.counts.iter() {
                if c as f64 * scale >= i as f64 {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl fmt::Display for Histogram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pairs: Vec<(&f32, &i64)> = self.bounds.iter().zip(&self.counts).collect();
        let (last, elements) = pairs.split_last().unwrap();
        for (b, c) in elements {
            write!(f, "{} < {} | ", c, b)?;
        }
        write!(f, "{} < ∞", last.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn some_histogram() -> Histogram {
        let mut hist = Histogram::new(0.0, 3.0, 4);
        hist.insert(0.0);
        hist.insert(99.9);
        return hist;
    }

    #[test]
    fn insert() {
        let hist = some_histogram();
        assert_eq!(hist.counts, vec![0, 1, 0, 0, 1]);
    }

    #[test]
    fn display() {
        let hist = some_histogram();
        let text = format!("{}", hist);
        assert_eq!(text, "0 < 0 | 1 < 1 | 0 < 2 | 0 < 3 | 1 < ∞");
    }

    #[test]
    fn reset() {
        let mut hist = some_histogram();
        assert_eq!(hist.counts, vec![0, 1, 0, 0, 1]);
        hist.reset();
        assert_eq!(hist.counts, vec![0, 0, 0, 0, 0]);
    }
}
