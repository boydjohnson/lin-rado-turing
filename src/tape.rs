use crate::types::Symbol;

#[derive(Debug, Clone)]
pub struct Tape<Symbol>(Vec<Symbol>);

impl<Sym: Symbol> Default for Tape<Sym> {
    fn default() -> Self {
        Self((0..500).map(|_| Sym::zero()).collect::<Vec<_>>())
    }
}

impl<Sym: Symbol> Tape<Sym> {
    pub fn read(&self, pos: usize) -> Option<&Sym> {
        self.0.get(pos)
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

    pub fn iter_between(&'_ self, first: usize, last: usize) -> impl Iterator<Item = Sym> + '_ {
        self.0[first..last.min(self.size() - 1)].iter().cloned()
    }

    pub fn iter_to(&'_ self, to: usize) -> impl Iterator<Item = Sym> + '_ {
        self.0[..to.min(self.size() - 1)].iter().cloned()
    }

    pub fn iter_from(&'_ self, from: usize) -> impl Iterator<Item = Sym> + '_ {
        self.0[from..].iter().cloned()
    }

    pub fn iter(&'_ self) -> impl Iterator<Item = Sym> + '_ {
        self.0.iter().cloned()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}
