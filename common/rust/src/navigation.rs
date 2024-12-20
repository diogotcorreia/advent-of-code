use std::iter::Step;
use std::ops::{Index, IndexMut};

use ndarray::Array2;
use num_traits::{CheckedAdd, CheckedMul, CheckedSub, Euclid, One, Signed, Zero};

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

impl Direction {
    pub fn get_all_orthogonal() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
    }

    pub fn to_mask(&self) -> u8 {
        match self {
            Direction::North => 1 << 0,
            Direction::East => 1 << 1,
            Direction::South => 1 << 2,
            Direction::West => 1 << 3,
            Direction::NorthEast => 1 << 4,
            Direction::SouthEast => 1 << 5,
            Direction::SouthWest => 1 << 6,
            Direction::NorthWest => 1 << 7,
        }
    }

    pub fn rotate_cw_90(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
        }
    }
    pub fn rotate_ccw_90(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::NorthEast => Direction::NorthWest,
            Direction::East => Direction::North,
            Direction::SouthEast => Direction::NorthEast,
            Direction::South => Direction::East,
            Direction::SouthWest => Direction::SouthEast,
            Direction::West => Direction::South,
            Direction::NorthWest => Direction::SouthWest,
        }
    }
}

/// A vector of any type in 2D space
#[derive(Debug, Hash, Clone, PartialEq, Eq, Default)]
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

impl<T: Euclid> Vec2D<T> {
    pub fn wrap_out_of_bounds(&self, upper_bound: &Self) -> Self {
        Self {
            x: self.x.rem_euclid(&upper_bound.x),
            y: self.y.rem_euclid(&upper_bound.y),
        }
    }
}

pub trait VecRadius<D>
where
    Self: Sized,
{
    fn all_points_manhattan_dist(&self, distance: D) -> impl Iterator<Item = (Self, D)>;
}

impl<T, D: Step + Copy + Signed> VecRadius<D> for Vec2D<T>
where
    Self: VecSum<Vec2D<D>>,
{
    fn all_points_manhattan_dist(&self, distance: D) -> impl Iterator<Item = (Self, D)> {
        (-distance..=distance).flat_map(move |dist_x| {
            let remaining_distance = distance - dist_x.abs();
            (-remaining_distance..=remaining_distance).flat_map(move |dist_y| {
                let delta_pos = Vec2D::<D>::new(dist_x, dist_y);

                self.vec_sum(&delta_pos)
                    .map(|p| (p, dist_x.abs() + dist_y.abs()))
            })
        })
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

impl<T> Index<&Vec2D<usize>> for Array2<T> {
    type Output = T;

    fn index(&self, index: &Vec2D<usize>) -> &Self::Output {
        self.index((index.y, index.x))
    }
}

impl<T> IndexMut<&Vec2D<usize>> for Array2<T> {
    fn index_mut(&mut self, index: &Vec2D<usize>) -> &mut Self::Output {
        self.index_mut((index.y, index.x))
    }
}
