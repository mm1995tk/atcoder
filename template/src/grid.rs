
#[derive(Debug, Clone)]
struct GridCell {
    focus: (usize, usize),
    h: usize,
    w: usize,
}

impl GridCell {
    fn left(&self, n: usize) -> Option<Self> {
        if self.focus.1 >= n {
            Some(Self {
                focus: (self.focus.0, self.focus.1 - n),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn right(&self, n: usize) -> Option<Self> {
        if self.focus.1 + n < self.w {
            Some(Self {
                focus: (self.focus.0, self.focus.1 + n),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn up(&self, n: usize) -> Option<Self> {
        if self.focus.0 >= n {
            Some(Self {
                focus: (self.focus.0 - n, self.focus.1),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn down(&self, n: usize) -> Option<Self> {
        if self.focus.0 + n < self.h {
            Some(Self {
                focus: (self.focus.0 + n, self.focus.1),
                ..self.clone()
            })
        } else {
            None
        }
    }
}