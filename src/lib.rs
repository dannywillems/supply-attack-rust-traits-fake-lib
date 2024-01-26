use std::ops::Add;

pub struct S1 {
    pub x: u64,
    pub y: u64,
}

pub struct S2 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

// Old version
// impl Add for S1 {
//     type Output = S1;

//     fn add(self, rhs: Self) -> Self::Output {
//         S1 {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

impl Add for S1 {
    type Output = S2;

    fn add(self, rhs: Self) -> Self::Output {
        S2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.x + rhs.y,
        }
    }
}
