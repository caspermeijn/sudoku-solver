/* Copyright (C) 2021 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct Board {
    cells: [Option<u8>; 9 * 9],
}

fn iter_into_array(mut iter: impl Iterator<Item = Option<u8>>) -> [Option<u8>; 9 * 9] {
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

impl From<[[u8; 9]; 9]> for Board {
    fn from(value: [[u8; 9]; 9]) -> Self {
        let value = value
            .iter()
            .flatten()
            .map(|n| if (1..=9).contains(n) { Some(*n) } else { None });

        Board {
            cells: iter_into_array(value),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_group(group: &[Option<u8>], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for cell in group {
                if let Some(n) = cell {
                    write!(f, "{}", n)?;
                } else {
                    write!(f, " ")?;
                }
            }
            Ok(())
        }

        fn fmt_row(row: &[Option<u8>], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut groups = row.chunks_exact(3);
            fmt_group(groups.next().unwrap(), f)?;
            write!(f, "|")?;
            fmt_group(groups.next().unwrap(), f)?;
            write!(f, "|")?;
            fmt_group(groups.next().unwrap(), f)?;
            writeln!(f)
        }

        let mut rows = self.cells.chunks_exact(9);
        for _ in 0..3 {
            fmt_row(rows.next().unwrap(), f)?;
        }
        writeln!(f, "---+---+---")?;
        for _ in 0..3 {
            fmt_row(rows.next().unwrap(), f)?;
        }
        writeln!(f, "---+---+---")?;
        for _ in 0..3 {
            fmt_row(rows.next().unwrap(), f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn board_from() {
        assert_eq!(
            Board {
                cells: [
                    None,
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(7),
                    Some(8),
                    Some(9),
                    Some(8),
                    Some(7),
                    Some(6),
                    Some(5),
                    Some(4),
                    Some(3),
                    Some(2),
                    Some(1),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(7),
                    Some(8),
                    Some(9),
                    Some(8),
                    Some(7),
                    Some(6),
                    Some(5),
                    Some(4),
                    Some(3),
                    Some(2),
                    Some(1),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(7),
                    Some(8),
                    Some(9),
                    Some(8),
                    Some(7),
                    Some(6),
                    Some(5),
                    Some(4),
                    Some(3),
                    Some(2),
                    Some(1),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ]
            },
            Board::from([
                [0, 1, 2, 3, 4, 5, 6, 7, 8],
                [9, 8, 7, 6, 5, 4, 3, 2, 1],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 2, 3, 4, 5, 6, 7, 8],
                [9, 8, 7, 6, 5, 4, 3, 2, 1],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 2, 3, 4, 5, 6, 7, 8],
                [9, 8, 7, 6, 5, 4, 3, 2, 1],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ])
        );
    }

    #[test]
    fn board_display() {
        let board = Board::from([
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 8, 7, 6, 5, 4, 3, 2, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 8, 7, 6, 5, 4, 3, 2, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 8, 7, 6, 5, 4, 3, 2, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);
        assert_eq!(
            "\
_12|345|678
987|654|321
___|___|___
---+---+---
_12|345|678
987|654|321
___|___|___
---+---+---
_12|345|678
987|654|321
___|___|___
",
            format!("{}", board).replace(" ", "_")
        );
    }
}
