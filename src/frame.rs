pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROwS);
        for _ in 0..NUM_ROwS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

// To be a drawable must implements the method draw
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
