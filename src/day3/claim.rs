use super::Point;
use super::Rectangle;

pub type ClaimId = u32;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Claim {
    id: ClaimId,
    rectangle: Rectangle,
}

impl Claim {
    pub fn new(id: ClaimId, top: u32, left: u32, width: u32, height: u32) -> Claim {
        Claim {
            id,
            rectangle: Rectangle::new(
                Point::new(left, top),
                Point::new(left + width, top + height),
            ),
        }
    }

    pub fn rectangle(&self) -> &Rectangle { &self.rectangle }

    pub fn id(&self) -> &ClaimId { &self.id }
}