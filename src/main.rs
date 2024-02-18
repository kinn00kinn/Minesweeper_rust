use std::io::{self, BufRead};
use crossterm::{cursor,execute, terminal, QueueableCommand};
use std::{thread, time};

struct Board {
    H: usize,
    W: usize,
    V: Vec<Vec<i32>>,
    S: Vec<Vec<i32>>,
}

impl Board {
    fn new(H: usize, W: usize) -> Board {
        Board {
            H,
            W,
            V: vec![vec![0; W]; H],
            S: vec![vec![-1; W]; H],
        }
    }

    fn set_bomb(&mut self, num: usize) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _i in 0..num {
            let x: usize = rng.gen_range(0, self.W);
            let y: usize = rng.gen_range(0, self.H);
            self.V[y][x] = 1;
        }
    }

    fn draw(&self) {
        for i in 0..self.H {
            for j in 0..self.W {
                if self.S[i][j] == -1 {
                    print!("{}", "o ");
                } else {
                    print!("{} ", self.S[i][j]);
                }
            }
            println!();
        }
    }

    fn open(&mut self, x: usize, y: usize) {
        if self.V[y][x] == 1 {
            panic!("This is Bomb!");
        } else if self.S[y][x] == -1 {
            let x_lab: Vec<isize> = vec![1, 1, 0, -1, -1, -1, 0, 1];
            let y_lab: Vec<isize> = vec![0, -1, -1, -1, 0, 1, 1, 1];
            let mut count = 0;
            for i in 0..x_lab.len() {
                let x_index = x as isize + x_lab[i];
                let y_index = y as isize + y_lab[i];
                if x_index >= 0 && x_index < self.W as isize && y_index >= 0 && y_index < self.H as isize {
                    if self.V[y_index as usize][x_index as usize] == 1 {
                        count += 1;
                    }
                }
            }
            self.S[y][x] = count;
        }
    }
    

    fn input_coordinates(&mut self) -> (usize, usize) {
        let stdin = io::stdin();
        loop {
            println!("Enter x coordinate (0-{}):", self.W - 1);
            let mut input_x = String::new();
            stdin.lock().read_line(&mut input_x).expect("Failed to read line");
            let x: usize = match input_x.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };
            if x >= self.W {
                println!("Invalid x coordinate. Please enter a number between 0 and {}.", self.W - 1);
                continue;
            }

            println!("Enter y coordinate (0-{}):", self.H - 1);
            let mut input_y = String::new();
            stdin.lock().read_line(&mut input_y).expect("Failed to read line");
            let y: usize = match input_y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };
            if y >= self.H {
                println!("Invalid y coordinate. Please enter a number between 0 and {}.", self.H - 1);
                continue;
            }
            
            return (x, y);
        }
    }
}

fn main() {
    //初期設定
    let mut board = Board::new(5, 5);
    board.set_bomb(5);
    
    //スタート画面
    print_start();

    //ゲーム
    loop {
        println!("Current Board:");
        board.draw();
        let (x, y) = board.input_coordinates();
        board.open(x, y);
    }
}

fn print_start(){
    print!("{}", terminal::Clear(terminal::ClearType::All));
   // クリアする空白の行をプリント
   for _ in 0..10 {
    println!();
}

// アニメーション
let logo = r#"
,,
`7MMM.     ,MMF'  db
  MMMb    dPMM
  M YM   ,M MM  `7MM  `7MMpMMMb.   .gP"Ya  ,pP"Ybd `7M'    ,A    `MF' .gP"Ya   .gP"Ya  `7MMpdMAo.  .gP"Ya  `7Mb,od8
  M  Mb  M' MM    MM    MM    MM  ,M'   Yb 8I   `"   VA   ,VAA   ,V  ,M'   Yb ,M'   Yb   MM   `Wb ,M'   Yb   MM' "'
  M  YM.P'  MM    MM    MM    MM  8M"""""" `YMMMa.    VA ,V  VA ,V   8M"""""" 8M""""""   MM    M8 8M""""""   MM
  M  `YM'   MM    MM    MM    MM  YM.    , L.   I8     VVV    VVV    YM.    , YM.    ,   MM   ,AP YM.    ,   MM
.JML. `'  .JMML..JMML..JMML  JMML. `Mbmmd' M9mmmP'      W      W      `Mbmmd'  `Mbmmd'   MMbmmd'   `Mbmmd' .JMML.
                                                                                         MM
                                                                                       .JMML.

"#;

let mut offset = 0;
loop {
    // ターミナルをクリア
    print!("\x1B[2J\x1B[1;1H");

    // ロゴを表示
    let mut lines = logo.lines();
    for _ in 0..offset {
        lines.next();
    }
    for _ in 0..10 {
        if let Some(line) = lines.next() {
            println!("{}", line);
        }
    }

    // アニメーションの速度を調整するために一定の時間待機
    thread::sleep(time::Duration::from_millis(100));

    // オフセットを更新し、ロゴをスクロール
    offset = (offset + 1) % 4;

    break;
}
}
