pub fn p1() -> usize {
    let mut e = Entry::init();
    while e.y != 2981 || e.x != 3075 {
        e = e.next()
    }
    e.value
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    value: usize,
    x: usize,
    y: usize,
}

impl Entry {
    fn init() -> Self {
        Self {
            value: 20151125,
            x: 1,
            y: 1,
        }
    }
    fn next(&self) -> Self {
        Self {
            value: ((self.value as u64 * 252533) % 33554393) as usize,
            x: if self.y == 1 { 1 } else { self.x + 1 },
            y: if self.y > 1 { self.y - 1 } else { self.x + 1 },
        }
    }
}
