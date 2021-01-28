use std::collections::BTreeMap;

pub struct Tape<Symbol>(BTreeMap<i64, Symbol>, i64, i64);

impl<Sym> Default for Tape<Sym> {
    fn default() -> Self {
        Tape(BTreeMap::default(), 0, 0)
    }
}

impl<Sym> Tape<Sym> {
    pub fn min(&self) -> i64 {
        self.1
    }

    pub fn max_plus_one(&self) -> i64 {
        self.2
    }

    pub fn read(&self, pos: i64) -> Option<&Sym> {
        self.0.get(&pos)
    }

    pub fn write(&mut self, pos: i64, symbol: Sym) {
        if self.1 > pos {
            self.1 = pos;
        }

        if self.2 <= pos {
            self.2 = pos + 1;
        }

        self.0.insert(pos, symbol);
    }

    pub fn iter_between(&self, first: i64, last: i64) -> impl Iterator<Item = &Sym> {
        assert!(first <= last);
        (first..last).filter_map(move |ref pos| self.0.get(pos))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Sym> {
        (self.1..self.2).map(move |ref pos| self.0.get(pos).expect("Logic error in Tape min, max"))
    }

    pub fn size(&self) -> usize {
        (self.2 - self.1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TwoSymbol;

    #[test]
    fn test_write_iter() {
        let mut tape = Tape::default();

        tape.write(0, TwoSymbol::Zero);

        tape.write(-1, TwoSymbol::One);

        tape.write(-2, TwoSymbol::One);

        tape.write(1, TwoSymbol::One);

        assert_eq!(
            tape.iter_between(-2, 0).collect::<Vec<_>>(),
            vec![&TwoSymbol::One, &TwoSymbol::One]
        );
    }

    #[test]
    fn test_write_iter2() {
        let mut tape = Tape::default();

        tape.write(0, TwoSymbol::One);

        tape.write(1, TwoSymbol::Zero);

        tape.write(2, TwoSymbol::One);

        assert_eq!(tape.iter().count(), 3);

        assert_eq!(tape.size(), 3);
    }

    #[test]
    fn test_write_size() {
        let mut tape = Tape::default();

        assert_eq!(tape.size(), 0);

        tape.write(0, TwoSymbol::Zero);

        assert_eq!(tape.size(), 1);

        tape.write(-1, TwoSymbol::Zero);

        assert_eq!(tape.size(), 2);

        tape.write(1, TwoSymbol::One);

        assert_eq!(tape.size(), 3);

        tape.write(2, TwoSymbol::One);

        assert_eq!(tape.size(), 4);
    }
}
