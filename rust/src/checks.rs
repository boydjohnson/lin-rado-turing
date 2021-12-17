fn recurr_check_init() -> (Snapshots<S, Sym>, Vec<i64>) {
    (BTreeMap::new(), vec![])
}

fn par_min_deviations(deviations: &[i64], dev: i64, pstep: usize) -> i64 {
    deviations[pstep..].par_iter().min().copied().unwrap_or(dev)
}

fn par_max_deviations(deviations: &[i64], dev: i64, pstep: usize) -> i64 {
    deviations[pstep..].par_iter().max().copied().unwrap_or(dev) + 1
}

fn par_recurr_check(
    &mut self,
    step: usize,
    snaps: &mut Snapshots<S, Sym>,
    deviations: &[i64],
    init: usize,
    beeps: &Beeps<S>,
    dev: i64,
) -> Option<Halt> {
    let action = (self.state, *self.read());

    if let Some(items) = snaps.get(&action).cloned() {
        let iter = items.par_iter();

        if let Some((pstep, step, pbeeps, ptape)) = iter
            .filter_map(|(pstep, pinit, pdev, ptape, pbeeps)| {
                let (prev, curr) = match dev.cmp(pdev) {
                    std::cmp::Ordering::Less => {
                        let dmax = Self::par_max_deviations(deviations, dev, *pstep);

                        let mut prev = ptape
                            .iter_to((*pinit as i64 + dmax) as usize)
                            .collect::<Vec<_>>();

                        let mut curr = self
                            .tape
                            .iter_to((init as i64 + dmax + dev - *pdev) as usize)
                            .collect::<Vec<_>>();

                        match curr.len().cmp(&prev.len()) {
                            Ordering::Greater => {
                                let mut prep = (0..(curr.len() - prev.len()))
                                    .map(|_| Sym::zero())
                                    .collect::<Vec<_>>();
                                prep.append(&mut prev);
                                prev = prep;
                            }
                            Ordering::Less => {
                                let mut prep = (0..(prev.len() - curr.len()))
                                    .map(|_| Sym::zero())
                                    .collect::<Vec<_>>();
                                prep.append(&mut curr);
                                curr = prep;
                            }
                            Ordering::Equal => (),
                        }

                        (prev, curr)
                    }
                    Ordering::Greater => {
                        let dmin = Self::par_min_deviations(deviations, dev, *pstep);

                        let from_prev = *pinit as i64 + dmin;

                        let mut prev = ptape.iter_from(from_prev).collect::<Vec<_>>();

                        let from_curr = init as i64 + dmin + dev - pdev;

                        let mut curr = self.tape.iter_from(from_curr).collect::<Vec<_>>();

                        match curr.len().cmp(&prev.len()) {
                            Ordering::Greater => {
                                let mut app = (0..(curr.len() - prev.len()))
                                    .map(|_| Sym::zero())
                                    .collect::<Vec<_>>();
                                prev.append(&mut app);
                            }
                            Ordering::Less => {
                                let mut app = (0..(prev.len() - curr.len()))
                                    .map(|_| Sym::zero())
                                    .collect::<Vec<_>>();
                                curr.append(&mut app);
                            }
                            Ordering::Equal => (),
                        }

                        (prev, curr)
                    }
                    Ordering::Equal => {
                        let dmax = Self::par_max_deviations(deviations, dev, *pstep);
                        let dmin = Self::par_min_deviations(deviations, dev, *pstep);

                        let from_prev = *pinit as i64 + dmin;

                        let prev = ptape
                            .iter_between(from_prev, *pinit as i64 + dmax)
                            .collect::<Vec<_>>();

                        let from_curr = init as i64 + dmin;

                        let curr = self
                            .tape
                            .iter_between(from_curr, init as i64 + dmax)
                            .collect::<Vec<_>>();

                        (prev, curr)
                    }
                };

                if prev == curr {
                    Some((pstep, step, pbeeps, ptape))
                } else {
                    None
                }
            })
            .min_by_key(|&(pstep, _, _, _)| pstep)
        {
            self.tape = ptape.clone();

            let reason = if pbeeps
                .keys()
                .all(|state| beeps.get(state) > pbeeps.get(state))
            {
                HaltReason::Recurr
            } else {
                HaltReason::Quasihalt
            };

            return Some(Halt::new(*pstep, reason(step - pstep)));
        }
    }

    snaps
        .entry(action)
        .and_modify(|v| v.push((step, init, dev, self.tape.clone(), beeps.clone())))
        .or_insert_with(|| vec![(step, init, dev, self.tape.clone(), beeps.clone())]);
    None
}

fn recurr_check(
    &mut self,
    step: usize,
    snaps: &mut Snapshots<S, Sym>,
    deviations: &[i64],
    init: usize,
    beeps: &Beeps<S>,
    dev: i64,
) -> Option<Halt> {
    let action = (self.state, self.read().copied().unwrap_or_else(Sym::zero));

    let mut iter = if let Some(items) = snaps.get(&action) {
        Either::Right(items.iter())
    } else {
        Either::Left(std::iter::empty())
    };

    if let Some((pstep, step, pbeeps, ptape)) = loop {
        if let Some((pstep, pinit, pdev, ptape, pbeeps)) = iter.next() {
            let (prev, curr) = match dev.cmp(pdev) {
                std::cmp::Ordering::Less => {
                    let dmax = deviations[*pstep..].iter().max().copied().unwrap_or(dev) + 1;

                    let to_prev = (*pinit as i64 + dmax) as usize;

                    let mut prev =
                        Box::new(ptape.iter_to(to_prev)) as Box<dyn Iterator<Item = Sym>>;

                    let to_curr = (init as i64 + dmax + dev - *pdev) as usize;

                    let mut curr =
                        Box::new(self.tape.iter_to(to_curr)) as Box<dyn Iterator<Item = Sym>>;

                    match to_curr.cmp(&to_prev) {
                        Ordering::Greater => {
                            let prep = (0..(to_curr - to_prev)).map(|_| Sym::zero());

                            prev = Box::new(prep.chain(prev)) as Box<dyn Iterator<Item = Sym>>;
                        }
                        Ordering::Less => {
                            let prep = (0..(to_prev - to_curr)).map(|_| Sym::zero());

                            curr = Box::new(prep.chain(curr)) as Box<dyn Iterator<Item = Sym>>;
                        }
                        Ordering::Equal => (),
                    }

                    (OfThree::One(prev), OfThree::One(curr))
                }
                Ordering::Greater => {
                    let dmin = deviations[*pstep..].iter().min().copied().unwrap_or(dev);

                    let from_prev = *pinit as i64 + dmin;

                    let prev = ptape.iter_from(from_prev);

                    let from_curr = init as i64 + dmin + dev - pdev;

                    let curr = self.tape.iter_from(from_curr);

                    (OfThree::Two(prev), OfThree::Two(curr))
                }
                Ordering::Equal => {
                    let dmax = deviations[*pstep..].iter().max().copied().unwrap_or(dev) + 1;
                    let dmin = deviations[*pstep..].iter().min().copied().unwrap_or(dev);

                    let from_prev = *pinit as i64 + dmin;

                    let prev = ptape.iter_between(from_prev, *pinit as i64 + dmax);

                    let from_curr = init as i64 + dmin;

                    let curr = self.tape.iter_between(from_curr, init as i64 + dmax);

                    (OfThree::Three(prev), OfThree::Three(curr))
                }
            };

            if prev.zip_longest(curr).all(|both| match both {
                Both(p, c) => p == c,
                Left(l) => l == Sym::zero(),
                Right(r) => r == Sym::zero(),
            }) {
                break Some((*pstep, step, pbeeps, ptape));
            }
        } else {
            break None;
        }
    } {
        self.tape = ptape.clone();

        let reason = if pbeeps
            .keys()
            .all(|state| beeps.get(state) > pbeeps.get(state))
        {
            HaltReason::Recurr
        } else {
            HaltReason::Quasihalt
        };

        return Some(Halt::new(pstep, reason(step - pstep)));
    }

    snaps
        .entry(action)
        .and_modify(|v| v.push((step, init, dev, self.tape.clone(), beeps.clone())))
        .or_insert_with(|| vec![(step, init, dev, self.tape.clone(), beeps.clone())]);
    None
}
