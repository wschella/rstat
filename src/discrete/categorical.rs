use core::*;
use rand::Rng;
use spaces::{Vector, discrete::Discrete};
use std::fmt;

pub type Multinoulli = Categorical;

#[derive(Debug, Clone)]
pub struct Categorical {
    pub ps: Vector<Probability>,
}

impl Categorical {
    pub fn new<P: Into<Probability>>(ps: Vec<P>) -> Categorical {
        let ps = Vector::from_vec(Probability::normalised(ps));

        Categorical { ps }
    }
}

impl Distribution for Categorical {
    type Support = Discrete;

    fn support(&self) -> Discrete { Discrete::new(self.ps.len() as usize) }

    fn cdf(&self, _: usize) -> Probability {
        unimplemented!()
    }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> usize {
        unimplemented!()
    }
}

impl DiscreteDistribution for Categorical {
    fn pmf(&self, i: usize) -> Probability {
        if i > self.ps.len() {
            panic!("Index must lie in the support: i < k.")
        }

        self.ps[i]
    }
}

impl Modes for Categorical {
    fn modes(&self) -> Vec<usize> {
        self.ps.iter().enumerate().fold((vec![0], self.ps[0]), |(mut modes, pmax), (j, p)| {
            use std::cmp::Ordering::*;

            match p.partial_cmp(&pmax) {
                Some(Less) => (modes, pmax),
                Some(Equal) => {
                    modes.push(j);

                    (modes, pmax)
                },
                Some(Greater) => {
                    modes.truncate(1);
                    modes[0] = j;

                    (modes, *p)
                },
                None => unreachable!(),
            }
        }).0
    }
}

impl fmt::Display for Categorical {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cat({})", self.ps)
    }
}
