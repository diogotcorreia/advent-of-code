use num_traits::{CheckedAdd, CheckedMul, CheckedSub, One, Signed, Zero};

/// The sign of an integer.
enum Sign {
    NonNeg,
    Neg,
}

/// Try to convert an integer to the unsigned variant and save the
/// previous sign of the integer.
trait TryIntoUnsigned
where
    Self: Sized,
{
    fn try_into_unsigned(self) -> Option<(Self, Sign)> {
        Some((self, Sign::NonNeg))
    }
}

macro_rules! impl_try_into_sign {
    ($t:ident) => {
        impl TryIntoUnsigned for $t {
            fn try_into_unsigned(self) -> Option<(Self, Sign)> {
                if self.is_negative() {
                    self.checked_neg().map(|v| (v, Sign::Neg))
                } else {
                    Some((self, Sign::NonNeg))
                }
            }
        }
    };
}

impl_try_into_sign!(i8);
impl_try_into_sign!(i16);
impl_try_into_sign!(i32);
impl_try_into_sign!(i64);
impl_try_into_sign!(i128);
impl_try_into_sign!(isize);
impl TryIntoUnsigned for u8 {}
impl TryIntoUnsigned for u16 {}
impl TryIntoUnsigned for u32 {}
impl TryIntoUnsigned for u64 {}
impl TryIntoUnsigned for u128 {}
impl TryIntoUnsigned for usize {}

/// Direction in 2D space
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// A vector of any type in 2D space
#[derive(Debug, Default)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

pub trait VecSum<Rhs>
where
    Self: Sized,
{
    fn vec_sum(&self, other: &Rhs) -> Option<Self>;
}

impl<T: CheckedAdd + CheckedSub, Rhs: TryIntoUnsigned + TryInto<T> + Copy> VecSum<Vec2D<Rhs>>
    for Vec2D<T>
{
    /// Sum two vectors, returning None if any of the integers overflows.
    fn vec_sum(&self, v: &Vec2D<Rhs>) -> Option<Self> {
        let x = match v.x.try_into_unsigned()? {
            (x, Sign::Neg) => self.x.checked_sub(&x.try_into().ok()?)?,
            (x, Sign::NonNeg) => self.x.checked_add(&x.try_into().ok()?)?,
        };
        let y = match v.y.try_into_unsigned()? {
            (y, Sign::Neg) => self.y.checked_sub(&y.try_into().ok()?)?,
            (y, Sign::NonNeg) => self.y.checked_add(&y.try_into().ok()?)?,
        };

        Some(Self { x, y })
    }
}

pub trait VecScale<Rhs>
where
    Self: Sized,
{
    /// Scale all fields of the vector by a given factor.
    /// Returns None if any field overflows.
    fn vec_scale(&self, factor: Rhs) -> Option<Self>;
}

impl<T: CheckedMul> VecScale<T> for Vec2D<T> {
    fn vec_scale(&self, factor: T) -> Option<Self> {
        let x = self.x.checked_mul(&factor)?;
        let y = self.y.checked_mul(&factor)?;
        Some(Self { x, y })
    }
}

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: PartialOrd + Default> Vec2D<T> {
    pub fn discard_out_of_bounds(self, lower_bound: &Self, upper_bound: &Self) -> Option<Self> {
        Some(self).filter(|vec| {
            vec.x >= lower_bound.x
                && vec.x < upper_bound.x
                && vec.y >= lower_bound.y
                && vec.y < upper_bound.y
        })
    }

    pub fn bind_to_map(self, upper_bound: &Self) -> Option<Self> {
        self.discard_out_of_bounds(&Self::default(), upper_bound)
    }
}

impl<T: One + Zero + Signed> From<Direction> for Vec2D<T> {
    /// Converts a direction to a unitary vector (i.e., all components are either -1, 0 or 1).
    /// The x component increases from North to South, while the y component
    /// increases from West to East.
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Vec2D {
                x: T::zero(),
                y: -T::one(),
            },
            Direction::NorthEast => Vec2D {
                x: T::one(),
                y: -T::one(),
            },
            Direction::East => Vec2D {
                x: T::one(),
                y: T::zero(),
            },
            Direction::SouthEast => Vec2D {
                x: T::one(),
                y: T::one(),
            },
            Direction::South => Vec2D {
                x: T::zero(),
                y: T::one(),
            },
            Direction::SouthWest => Vec2D {
                x: -T::one(),
                y: T::one(),
            },
            Direction::West => Vec2D {
                x: -T::one(),
                y: T::zero(),
            },
            Direction::NorthWest => Vec2D {
                x: -T::one(),
                y: -T::one(),
            },
        }
    }
}
