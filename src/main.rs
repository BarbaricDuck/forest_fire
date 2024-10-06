use std::{io::{stdout, BufWriter, Stdout, StdoutLock, Write}, thread::sleep, time::Duration};
use crossterm::{cursor, QueueableCommand, ExecutableCommand, terminal};

#[derive(Debug, Clone, Copy)]
struct Pixel {
    value: u8,
    next_value: u8,
}

impl Pixel {
    // Constructor for Pixel
    fn new(value: u8) -> Self {
        Self { value, next_value: Pixel::try_incr(value) }
    }

    // Method to get the character representation of the pixel
    fn to_char(self) -> char {
        " .@%#:*+=-.".chars().nth(self.value.into()).unwrap()
    }

    fn try_incr(value: u8) -> u8 {
        if value > 9 {
            return 0;
        }
        value + 1
    }

    fn incr(&mut self) {
        self.value = Pixel::try_incr(self.value);
    }
}

struct Board {
    data: Vec<Vec<Pixel>>,
    f: f64,
    p: f64,
}

impl Board {
    fn new(rows: usize, cols: usize, f: f64, p: f64) -> Self {
        let data: Vec<Vec<Pixel>> = vec![vec![Pixel::new(0); cols]; rows];
        Self { data, f, p}
    }

    fn print(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }

    fn display(&self, writer: &mut BufWriter<StdoutLock<'_>>) {
        writer.queue(cursor::MoveTo(0, 0)).unwrap();
        writer.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        for row in &self.data {
            for col in row {
                write!(writer, "{}", col.to_char()).unwrap();
            }
            write!(writer, "\n").unwrap();
        }
        writer.flush().unwrap();
    }

    // fn resize(&mut self, rows: usize, cols: usize) {
    //     for row in &mut self.data {
    //         row.resize(cols, 0);
    //     }

    //     self.data.resize(rows, vec![0; cols]);
    // }

    fn increment_whole(&mut self) {
        for row in &mut self.data { // Get mutable references to rows
            for col in row.iter_mut() { // Get mutable references to cols
                col.incr(); // Dereference and increment the value
            }
        }
    }
}

// fn round_down_to_nearest_2(number: usize) -> usize {
//     ((number as f64 / 2.0).floor() * 2.0) as usize
// }

fn main() {
    let stdout = stdout();
    let mut writer: BufWriter<StdoutLock<'_>> = BufWriter::new(stdout.lock());
    writer.queue(terminal::EnterAlternateScreen).unwrap();
    let (max_x, max_y) = terminal::size().unwrap();
    let mut board = Board::new((max_y).into(), (max_x).into(), 0.0001, 0.001);
    board.display(&mut writer);
    board.increment_whole();
    sleep(Duration::from_secs(1));
    writer.execute(terminal::LeaveAlternateScreen).unwrap();
    

    
}