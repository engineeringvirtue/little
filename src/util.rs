pub type Pos = (i32, i32);

pub struct Region {
    pub from: Pos,
    pub to: Pos
}

impl Region {
    pub fn new(from: Pos, to: Pos) -> Self {
        Region {from, to}
    }

    pub fn inside(&self, pos: Pos) -> bool {
        pos.0 >= self.from.0 && pos.1 >= self.from.1
            && pos.0 <= self.to.0 && pos.1 <= self.to.1
    }
}