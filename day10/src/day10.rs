#[derive(Debug, Copy, Clone, PartialEq)]
enum TileContents {
    Missing,
    Start,
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthEastPipe,
    SouthWestPipe,
}

impl From<char> for TileContents {
    fn from(item: char) -> Self {
        match item {
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            '7' => Self::SouthWestPipe,
            'J' => Self::NorthWestPipe,
            'F' => Self::SouthEastPipe,
            'L' => Self::NorthEastPipe,
            'S' => Self::Start,
            _ => Self::Missing,
        }
    }
}

impl TileContents {
    fn connects_north(&self) -> bool {
        match self {
            Self::Missing => false,
            Self::VerticalPipe => true,
            Self::HorizontalPipe => false,
            Self::NorthEastPipe => true,
            Self::NorthWestPipe => true,
            Self::SouthEastPipe => false,
            Self::SouthWestPipe => false,
            Self::Start => true,
        }
    }

    fn connects_south(&self) -> bool {
        match self {
            Self::Missing => false,
            Self::VerticalPipe => true,
            Self::HorizontalPipe => false,
            Self::NorthEastPipe => false,
            Self::NorthWestPipe => false,
            Self::SouthEastPipe => true,
            Self::SouthWestPipe => true,
            Self::Start => true,
        }
    }

    fn connects_east(&self) -> bool {
        match self {
            Self::Missing => false,
            Self::VerticalPipe => false,
            Self::HorizontalPipe => true,
            Self::NorthEastPipe => true,
            Self::NorthWestPipe => false,
            Self::SouthEastPipe => true,
            Self::SouthWestPipe => false,
            Self::Start => true,
        }
    }

    fn connects_west(&self) -> bool {
        match self {
            Self::Missing => false,
            Self::VerticalPipe => false,
            Self::HorizontalPipe => true,
            Self::NorthEastPipe => false,
            Self::NorthWestPipe => true,
            Self::SouthEastPipe => false,
            Self::SouthWestPipe => true,
            Self::Start => true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tile {
    point: Point,
    tile_contents: TileContents,
}

#[derive(Debug, PartialEq)]
pub struct Matrix {
    points: Vec<Vec<TileContents>>,
}

struct ConnectionIterator<'a> {
    state: usize,
    origin: Point,
    matrix: &'a Matrix,
}

impl<'a> Iterator for ConnectionIterator<'a> {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        if self.state > 3 {
            return None;
        }

        match self.state {
            0 => {
                let this = self
                    .matrix
                    .get_tile(&self.origin)
                    .expect("expect iterator to be created on a valid point");
                if let Some(other) = self.matrix.get_north(&this.point) {
                    if this.tile_contents.connects_north() && other.tile_contents.connects_south() {
                        self.state += 1;
                        return Some(other);
                    }
                }
                self.state += 1;
                self.next()
            }
            1 => {
                let this = self
                    .matrix
                    .get_tile(&self.origin)
                    .expect("expect iterator to be created on a valid point");
                if let Some(other) = self.matrix.get_east(&this.point) {
                    if this.tile_contents.connects_east() && other.tile_contents.connects_west() {
                        self.state += 1;
                        return Some(other);
                    }
                }
                self.state += 1;
                self.next()
            }
            2 => {
                let this = self
                    .matrix
                    .get_tile(&self.origin)
                    .expect("expect iterator to be created on a valid point");
                if let Some(other) = self.matrix.get_south(&this.point) {
                    if this.tile_contents.connects_south() && other.tile_contents.connects_north() {
                        self.state += 1;
                        return Some(other);
                    }
                }
                self.state += 1;
                self.next()
            }
            3 => {
                let this = self
                    .matrix
                    .get_tile(&self.origin)
                    .expect("expect iterator to be created on a valid point");
                if let Some(other) = self.matrix.get_west(&this.point) {
                    if this.tile_contents.connects_west() && other.tile_contents.connects_east() {
                        self.state += 1;
                        return Some(other);
                    }
                }
                self.state += 1;
                self.next()
            }
            _ => None,
        }
    }
}

impl From<&str> for Matrix {
    fn from(item: &str) -> Self {
        let len = item.find('\n').unwrap();
        let mut matrix = Matrix {
            points: Vec::with_capacity(len),
        };

        item.lines().for_each(|l| {
            let mut tiles = Vec::with_capacity(len);
            l.chars().for_each(|c| tiles.push(TileContents::from(c)));
            matrix.points.push(tiles);
        });

        matrix
    }
}

impl Matrix {
    fn find_start(&self) -> Tile {
        for (y, r) in self.points.iter().enumerate() {
            for (x, c) in r.iter().enumerate() {
                if c == &TileContents::Start {
                    return Tile {
                        point: Point { x, y },
                        tile_contents: *c,
                    };
                }
            }
        }
        panic!("should always find a start value in the data")
    }

    fn neighbor_iter(&self, p: &Point) -> ConnectionIterator {
        ConnectionIterator {
            matrix: self,
            state: 0,
            origin: *p,
        }
    }

    fn get_tile(&self, p: &Point) -> Option<Tile> {
        let len = self.points.len();
        if p.x < len && p.y < len {
            return Some(Tile {
                point: *p,
                tile_contents: self.points[p.y][p.x],
            });
        }
        None
    }

    fn get_north(&self, p: &Point) -> Option<Tile> {
        self.get_tile(&Point { x: p.x, y: p.y - 1 })
    }

    fn get_south(&self, p: &Point) -> Option<Tile> {
        self.get_tile(&Point { x: p.x, y: p.y + 1 })
    }

    fn get_west(&self, p: &Point) -> Option<Tile> {
        self.get_tile(&Point { x: p.x - 1, y: p.y })
    }

    fn get_east(&self, p: &Point) -> Option<Tile> {
        self.get_tile(&Point { x: p.x + 1, y: p.y })
    }
}

pub fn solve_part_one(input: &str) -> u32 {
    let matrix = Matrix::from(input);
    let start_tile = matrix.find_start();
    find_loop_length(&matrix, &start_tile, &start_tile, 0).unwrap()
}

pub fn find_loop_length(matrix: &Matrix, current: &Tile, last: &Tile, len: u32) -> Option<u32> {
    let mut iter = matrix.neighbor_iter(&current.point).peekable();
    iter.peek()?;
    for tile in iter {
        if tile.tile_contents == TileContents::Start {
            return Some(len.div_ceil(2));
        }
        if tile != *last {
            let found = find_loop_length(matrix, &tile, current, len + 1);
            if found.is_some() {
                return found;
            }
        }
    }
    None
}

pub fn solve_part_two(input: &str) -> i64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_EXAMPLE: &str = r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

    #[test]
    fn test_iterator() {
        let m = Matrix::from(TEST_EXAMPLE);
        let actual = m.neighbor_iter(&m.find_start().point).collect_vec();
        let expected = [
            Tile {
                point: Point { x: 2, y: 2 },
                tile_contents: TileContents::HorizontalPipe,
            },
            Tile {
                point: Point { x: 1, y: 3 },
                tile_contents: TileContents::VerticalPipe,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(TEST_EXAMPLE), 4);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(TEST_EXAMPLE), 2);
    }
}
