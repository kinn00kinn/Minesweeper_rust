use std::{string, vec};
use rand::Rng;

struct Board {
    //盤面を管理するクラスのフィールド
    H: i32,
    W: i32,
    V: Vec<Vec<i32>>,
}

impl Board {
    //盤面を管理するクラスのメソッド
    fn new(H: i32, W: i32) -> Board {
        Board {
            H,
            W,
            V: vec![vec![0; W as usize]; H as usize],
        }
    }

    fn set_bomb(&mut self, num: i32) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _i in 0..num {
            let x: usize = rng.gen_range(0, self.W as usize);
            let y: usize = rng.gen_range(0, self.H as usize);
            self.V[y][x] = 1;
        }
    }

    fn draw(&self) {
        for i in 0..self.H {
            for j in 0..self.W {
                if self.V[i as usize][j as usize] == 0 {
                    print!("{}", "o ");
                } else if self.V[i as usize][j as usize] == 1 {
                    print!("{}", "\x1b[38;5;1mx\x1b[m ");
                }
            }
            println!();
        }
    }
}



fn main() {
    let H = 5;
    let W = 7;

    let mut board1 = Board::new(H, W);
    board1.set_bomb(10);
    board1.draw();
}

