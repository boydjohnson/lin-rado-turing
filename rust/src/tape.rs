use crate::types::{Direction, Symbol};

#[derive(Debug, Clone)]
pub struct Tape<Symbol>(Vec<Symbol>, usize, i64);

impl<Sym: Symbol> Default for Tape<Sym> {
    fn default() -> Self {
        Self((0..1).map(|_| Sym::zero()).collect::<Vec<_>>(), 0, 0)
    }
}

impl<Sym: Symbol> Tape<Sym> {
    pub fn read(&self) -> Sym {
        self.0.get(self.1).copied().unwrap_or_else(|| Sym::zero())
    }

    pub fn insert(&mut self) {
        self.0.insert(0, Sym::zero());
    }

    pub fn write(&mut self, pos: usize, symbol: Sym) {
        if let Some(val) = self.0.get_mut(pos) {
            *val = symbol;
        } else {
            self.0.push(symbol);
        }
    }

    pub fn iter_between(&'_ self, first: i64, last: i64) -> impl Iterator<Item = Sym> + '_ {
        (first..last).map(move |i| {
            if i < 0 {
                Sym::zero()
            } else {
                self.0.get(i as usize).cloned().unwrap_or_else(Sym::zero)
            }
        })
    }

    pub fn iter_to(&'_ self, to: usize) -> impl Iterator<Item = Sym> + '_ {
        (0..to).map(move |i| self.0.get(i).cloned().unwrap_or_else(Sym::zero))
    }

    pub fn iter_from(&'_ self, from: i64) -> impl Iterator<Item = Sym> + '_ {
        (from..self.0.len() as i64).map(move |i| {
            if i < 0 {
                Sym::zero()
            } else {
                self.0.get(i as usize).cloned().unwrap_or_else(Sym::zero)
            }
        })
    }

    pub fn iter(&'_ self) -> impl Iterator<Item = Sym> + '_ {
        self.0.iter().cloned()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl<Sym: Symbol> ITape<Sym> for Tape<Sym> {
    fn read(&self) -> Sym {
        self.read()
    }

    fn marks(&self) -> usize {
        self.0.iter().filter(|&&s| s != Sym::zero()).count()
    }

    fn write_symbol(&mut self, direction: Direction, symbol: Sym) -> usize {
        self.write(self.1, symbol);
        match direction {
            Direction::Left => {
                if self.1 == 0 {
                    self.insert();
                } else {
                    self.1 -= 1;
                }
            }
            Direction::Right => {
                self.1 += 1;
            }
        }
        1
    }
}

pub trait ITape<Sym>: Default + Clone {
    fn read(&self) -> Sym;

    fn marks(&self) -> usize;

    fn write_symbol(&mut self, direction: Direction, symbol: Sym) -> usize;
}
