use std::collections::*;

use super::*;

type ClaimCount = u32;

pub struct Fabric {
    cells: HashMap<Point, ClaimCount>
}

impl Fabric {
    pub fn new() -> Fabric {
        Fabric {
            cells: HashMap::new()
        }
    }

    pub fn take(&mut self, claim: &Claim) {
        claim.rectangle()
            .point_matrix()
            .iter()
            .for_each(|point| *self.cells.entry(*point).or_insert(0) += 1)
    }

    pub fn count_used_area<F>(&self, predicate: F) -> Area where F: (Fn(&u32) -> bool) {
        self.cells
            .iter()
            .map(|(_, count)| count.clone())
            .filter(predicate)
            .count() as u32
    }
}