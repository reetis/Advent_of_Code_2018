use std::collections::*;

use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Rectangle {
        assert!(top_left.x() < bottom_right.x());
        assert!(top_left.y() < bottom_right.y());
        Rectangle { top_left, bottom_right }
    }

    pub fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        let top =
            if self.top_left.y() <= other.bottom_right.y() &&
                self.top_left.y() >= other.top_left.y() {
                self.top_left.y()
            } else if other.top_left.y() <= self.bottom_right.y() &&
                other.top_left.y() >= self.top_left.y() {
                other.top_left.y()
            } else {
                return None;
            };

        let bottom =
            if self.bottom_right.y() <= other.bottom_right.y() &&
                self.bottom_right.y() >= other.top_left.y() {
                self.bottom_right.y()
            } else if other.bottom_right.y() <= self.bottom_right.y() &&
                other.bottom_right.y() >= self.top_left.y() {
                other.bottom_right.y()
            } else {
                return None;
            };

        let left =
            if self.top_left.x() <= other.bottom_right.x() &&
                self.top_left.x() >= other.top_left.x() {
                self.top_left.x()
            } else if other.top_left.x() <= self.bottom_right.x() &&
                other.top_left.x() >= self.top_left.x() {
                other.top_left.x()
            } else {
                return None;
            };

        let right =
            if self.bottom_right.x() <= other.bottom_right.x() &&
                self.bottom_right.x() >= other.top_left.x() {
                self.bottom_right.x()
            } else if other.bottom_right.x() <= self.bottom_right.x() &&
                other.bottom_right.x() >= self.top_left.x() {
                other.bottom_right.x()
            } else {
                return None;
            };

        if bottom > top && right > left {
            return Some(
                Rectangle::new(
                    Point::new(left.clone(), top.clone()),
                    Point::new(right.clone(), bottom.clone()),
                )
            );
        } else {
            return None;
        }
    }

    pub fn area(&self) -> Area {
        let width = self.bottom_right.x() - self.top_left.x();
        let height = self.bottom_right.y() - self.top_left.y();

        width * height
    }

    pub fn point_matrix(&self) -> HashSet<Point> {
        let mut result = HashSet::new();

        for x in self.top_left.x().clone()..self.bottom_right.x().clone() {
            for y in self.top_left.y().clone()..self.bottom_right.y().clone() {
                result.insert(Point::new(x, y));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_area() {
        let input = Rectangle::new(
            Point::new(1, 2),
            Point::new(3, 5),
        );

        assert_eq!(input.area(), 6)
    }

    #[test]
    fn should_calculate_intersection() {
        let input1 = Rectangle::new(
            Point::new(1, 2),
            Point::new(3, 5),
        );

        let input2 = Rectangle::new(
            Point::new(2, 4),
            Point::new(4, 6),
        );
        let result = input1.intersection(&input2).unwrap();

        assert_eq!(
            result,
            Rectangle::new(
                Point::new(2, 4),
                Point::new(3, 5),
            )
        )
    }


    #[test]
    fn should_create_point_matrix() {
        let input = Rectangle::new(
            Point::new(1, 2),
            Point::new(3, 4),
        );

        let expected_result: HashSet<Point> = hashset![
            Point::new(1, 2),
            Point::new(1, 3),
            Point::new(2, 2),
            Point::new(2, 3),
        ];

        assert_eq!(input.point_matrix(), expected_result)
    }
}