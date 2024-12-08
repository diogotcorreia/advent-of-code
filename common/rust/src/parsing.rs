use itertools::Itertools;
use ndarray::Array2;

use crate::{navigation::Vec2D, DayError};

pub fn try_parse_2d_array<T: TryFrom<char>>(
    lines: impl Iterator<Item = String>,
) -> Result<Array2<T>, DayError> {
    let mut lines = lines.peekable();
    let width = lines
        .peek()
        .ok_or(DayError::GenericParseErr("input is empty"))?
        .len();
    let board = lines
        .flat_map(|row| {
            row.chars()
                .map(|c| {
                    T::try_from(c)
                        .map_err(|_| DayError::GenericParseErr("can't parse character in map"))
                })
                .collect_vec()
        })
        .process_results(|it| it.collect_vec())?;

    Array2::from_shape_vec((board.len() / width, width), board).map_err(|_| {
        DayError::GenericParseErr(
            "lines in input have different lengths, while a rectangle input was expected",
        )
    })
}

pub trait MaybeParseChar
where
    Self: Sized,
{
    type Error;
    fn maybe_parse_char(pos: Vec2D<usize>, c: char) -> Result<Option<Self>, Self::Error>;
}

pub fn try_parse_sparse_2d_array<T: MaybeParseChar>(
    lines: impl Iterator<Item = String>,
) -> Result<(Vec<T>, Vec2D<usize>), DayError> {
    let mut lines = lines.peekable();
    let width = lines
        .peek()
        .ok_or(DayError::GenericParseErr("input is empty"))?
        .len();
    let mut height = 0;
    let list = lines
        .enumerate()
        .flat_map(|(y, row)| {
            height += 1;
            row.chars()
                .enumerate()
                .map(|(x, c)| {
                    T::maybe_parse_char(Vec2D::<usize>::new(x, y), c)
                        .map_err(|_| DayError::GenericParseErr("can't parse character in map"))
                })
                .collect_vec()
        })
        .process_results(|it| it.flatten().collect_vec())?;

    Ok((list, Vec2D::<usize>::new(width, height)))
}
