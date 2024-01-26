use std::ops::Add;

pub struct S1 {
    pub x: u64,
    pub y: u64,
}

impl Add for S1 {
    type Output = S1;

    fn add(self, rhs: Self) -> Self::Output {
        S1 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
