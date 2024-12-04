use itertools::Itertools;
use ndarray::Array2;

use crate::DayError;

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
