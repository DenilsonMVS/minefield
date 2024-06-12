
pub enum Fail {
    OutOfBounds,
    MineExploded,
}

pub struct Field {
    mines: Vec<Vec<bool>>,
    can_see: Vec<Vec<bool>>,
}

impl Field {
    const MINES_RATIO: f32 = 0.15;
    const NEIGHBORS: [[isize; 2]; 8] = [
        [-1, -1],
        [-1,  0],
        [-1,  1],
        [ 0, -1],
        [ 0,  1],
        [ 1, -1],
        [ 1,  0],
        [ 1,  1],
    ];

    fn generate_mines(width: usize, height: usize) -> Vec<Vec<bool>> {
        
        use rand::prelude::*;
        let mut rng = thread_rng();
        
        let mut quads = vec![vec![false; width]; height];

        let total_quads = width * height;
        let total_mines = (total_quads as f32 * Self::MINES_RATIO) as usize;
        for _ in 0..total_mines {
            loop {
                let y = rng.gen_range(0..height);
                let x = rng.gen_range(0..width);
                if !quads[y][x] {
                    quads[y][x] = true;
                    break;
                }
            }
        }
        return quads;
    }

    fn get_width(&self) -> usize {
        return self.mines[0].len();
    }

    fn get_height(&self) -> usize {
        return self.mines.len();
    }

    fn is_inside(&self, x: usize, y: usize) -> bool {
        return x < self.get_width() && y < self.get_height();
    }

    fn get_amount(&self, x: usize, y: usize) -> usize {
        return Self::NEIGHBORS.iter().fold(0, |count, offset| {
            let nx = x.wrapping_add(offset[0] as usize);
            let ny = y.wrapping_add(offset[1] as usize);
            if self.is_inside(nx, ny) && self.mines[ny][nx] {
                return count + 1;
            } else {
                return count;
            }
        });
    }

    fn get_view_character(&self, x: usize, y: usize) -> Option<char> {
        if !self.is_inside(x, y) {
            return None;
        }

        if !self.can_see[y][x] {
            return Some(' ');
        }

        if self.mines[y][x] {
            return Some('*');
        }

        let amount = self.get_amount(x, y);
        if amount == 0 {
            return Some(' ');
        } else {
            return Some((b'0' + amount as u8) as char);
        }
    }

    pub fn new(width: std::num::NonZeroUsize, height: std::num::NonZeroUsize) -> Self {
        let mines = Self::generate_mines(width.get(), height.get());
        let can_see = vec![vec![false; width.get()]; height.get()];
        return Self { mines, can_see };
    }

    fn from_raw(mines: Vec<Vec<bool>>, can_see: Vec<Vec<bool>>) -> Option<Self> {
        if mines.len() != can_see.len() || mines.is_empty() {
            return None;
        }

        for i in 0..mines.len() {
            if mines[i].len() != can_see[i].len() || mines[i].is_empty() {
                return None;
            }
        }

        return Some(Self { mines, can_see });
    }

    pub fn show(&self) {
        print!("#");
        for _ in 0..self.get_width() {
            print!("#");
        }
        println!("#");

        for y in 0..self.get_height() {
            print!("#");
            for x in 0..self.get_width() {
                print!("{}", self.get_view_character(x, y).unwrap());
            }
            println!("#");
        }

        print!("#");
        for _ in 0..self.get_width() {
            print!("#");
        }
        println!("#");
    }

    fn spread(&mut self, x: usize, y: usize) {
        if !self.is_inside(x, y) || self.can_see[y][x] {
            return;
        }
        self.can_see[y][x] = true;

        if self.mines[y][x] || self.get_amount(x, y) != 0 {
            return;
        }

        Self::NEIGHBORS.iter().for_each(|neighbor| {
            let nx = x.wrapping_add(neighbor[0] as usize);
            let ny = y.wrapping_add(neighbor[1] as usize);
            self.spread(nx, ny);
        });
    }

    pub fn click(&mut self, x: usize, y: usize) -> Result<(), Fail> {
        if !self.is_inside(x, y) {
            return Err(Fail::OutOfBounds);
        }

        self.spread(x, y);

        if self.mines[y][x] {
            return Err(Fail::MineExploded);
        }

        return Ok(());
    }

    pub fn all_non_mine_visible(&self) -> bool {
        return (0..self.get_height()).all(|y| {
            (0..self.get_width()).all(|x| {
                self.mines[y][x] || self.can_see[y][x]
            })
        });
    }
}


#[cfg(test)]
mod tests {
    use std::vec;

    use crate::field::Field;

    #[test]
    fn test_field_dimensions() {
        let field = Field::new(
            std::num::NonZeroUsize::new(10).unwrap(), 
            std::num::NonZeroUsize::new(20).unwrap()
        );
        assert_eq!(field.get_width(), 10);
        assert_eq!(field.get_height(), 20);
    }

    #[test]
    fn test_out_of_bounds() {
        let field = Field::new(
            std::num::NonZeroUsize::new(10).unwrap(), 
            std::num::NonZeroUsize::new(20).unwrap()
        );

        assert_eq!(field.is_inside(0, 0), true);
        assert_eq!(field.is_inside(9, 0), true);
        assert_eq!(field.is_inside(9, 19), true);
        assert_eq!(field.is_inside(0, 19), true);
        assert_eq!(field.is_inside(0, 20), false);
        assert_eq!(field.is_inside(10, 0), false);
    }

    fn create_test_field(visible: bool) -> Field {
        let mines = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false], // 1
            vec![ true, false, false], // 2
            vec![ true, false, false], // 3
            vec![ true, false, false], // 4
            vec![ true, false,  true], // 5
            vec![ true, false,  true], // 6
            vec![ true, false,  true], // 7
            vec![ true,  true,  true],
            vec![ true, false,  true], // 8
            vec![ true,  true,  true],
        ];
        let can_see = vec![
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
            vec![ visible, visible, visible],
        ];
        return Field::from_raw(mines, can_see).unwrap();
    }

    #[test]
    fn test_zero_mines() {
        let field = create_test_field(false);
        assert_eq!(field.get_amount(0, 0), 0);
        assert_eq!(field.get_amount(0, 1), 0);
        assert_eq!(field.get_amount(0, 2), 0);
        assert_eq!(field.get_amount(1, 0), 0);
        assert_eq!(field.get_amount(1, 1), 0);
        assert_eq!(field.get_amount(1, 2), 0);
    }

    #[test]
    fn test_one_mine() {
        let field = create_test_field(false);
        assert_eq!(field.get_amount(1, 4), 1);
    }

    #[test]
    fn test_two_mines() {
        let field = create_test_field(false);
        assert_eq!(field.get_amount(1, 5), 2);
    }

    #[test]
    fn test_three_mines() {
        let field = create_test_field(false);
        assert_eq!(field.get_amount(1, 6), 3);
    }

    #[test]
    fn test_four_mines() {
        let field = create_test_field(false);
        assert_eq!(field.get_amount(1, 7), 4);
    }

    #[test]
    fn test_five_mines() {
        let field = create_test_field(false);
        assert_eq!(field.get_amount(1, 8), 5);
    }

    #[test]
    fn test_six_mines() {
        let field = create_test_field(false);
        assert_eq!(field.get_amount(1, 9), 6);
    }

    #[test]
    fn test_seven_mine() {
        let field = create_test_field(false);
        assert_eq!(field.get_amount(1, 10), 7);
    }

    #[test]
    fn test_eight_mines() {
        let field = create_test_field(false);
        assert_eq!(field.get_amount(1, 12), 8);
    }

    #[test]
    fn test_not_visible_empty_quad() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(0, 0).unwrap(), '-');
    }

    #[test]
    fn test_not_visible_mine() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(0, 5).unwrap(), '-');
    }

    #[test]
    fn test_not_visible_one() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(1, 4).unwrap(), '-');
    }

    #[test]
    fn test_not_visible_two() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(1, 5).unwrap(), '-');
    }

    #[test]
    fn test_not_visible_three() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(1, 6).unwrap(), '-');
    }

    #[test]
    fn test_not_visible_four() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(1, 7).unwrap(), '-');
    }

    #[test]
    fn test_not_visible_five() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(1, 8).unwrap(), '-');
    }

    #[test]
    fn test_not_visible_six() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(1, 9).unwrap(), '-');
    }

    #[test]
    fn test_not_visible_seven() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(1, 10).unwrap(), '-');
    }

    #[test]
    fn test_not_visible_eight() {
        let field = create_test_field(false);
        assert_eq!(field.get_view_character(1, 12).unwrap(), '-');
    }

    #[test]
    fn test_visible_empty_quad() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(0, 0).unwrap(), ' ');
    }

    #[test]
    fn test_visible_mine() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(0, 5).unwrap(), '*');
    }

    #[test]
    fn test_visible_one() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(1, 4).unwrap(), '1');
    }

    #[test]
    fn test_visible_two() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(1, 5).unwrap(), '2');
    }

    #[test]
    fn test_visible_three() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(1, 6).unwrap(), '3');
    }

    #[test]
    fn test_visible_four() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(1, 7).unwrap(), '4');
    }

    #[test]
    fn test_visible_five() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(1, 8).unwrap(), '5');
    }

    #[test]
    fn test_visible_six() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(1, 9).unwrap(), '6');
    }

    #[test]
    fn test_visible_seven() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(1, 10).unwrap(), '7');
    }

    #[test]
    fn test_visible_eight() {
        let field = create_test_field(true);
        assert_eq!(field.get_view_character(1, 12).unwrap(), '8');
    }

    #[test]
    fn test_spread() {
        let mut field = create_test_field(false);
        let _ = field.click(0, 0);
        let expected_result = [
            [' ', ' ', ' '],
            [' ', ' ', ' '],
            [' ', ' ', ' '],
            [' ', ' ', ' '],
            ['1', '1', ' '],
            ['-', '2', ' '],
            ['-', '3', ' '],
            ['-', '4', '1'],
            ['-', '-', '-'],
            ['-', '-', '-'],
            ['-', '-', '-'],
            ['-', '-', '-'],
            ['-', '-', '-'],
            ['-', '-', '-'],
        ];
        for y in 0..field.get_height() {
            for x in 0..field.get_width() {
                assert_eq!(expected_result[y][x], field.get_view_character(x, y).unwrap());
            }
        }

    }
}
