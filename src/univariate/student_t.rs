use crate::{
    consts::PI,
    prelude::*,
    validation::{Validator, Result},
};
use rand::Rng;
use spaces::real::Reals;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct StudentT {
    pub nu: f64,
}

impl StudentT {
    pub fn new(nu: f64) -> Result<StudentT> {
        Validator
            .require_positive_real(nu)
            .map(|_| StudentT::new_unchecked(nu))
    }

    pub fn new_unchecked(nu: f64) -> StudentT {
        StudentT { nu }
    }
}

impl Into<rand_distr::StudentT<f64>> for StudentT {
    fn into(self) -> rand_distr::StudentT<f64> {
        rand_distr::StudentT::new(self.nu).unwrap()
    }
}

impl Into<rand_distr::StudentT<f64>> for &StudentT {
    fn into(self) -> rand_distr::StudentT<f64> {
        rand_distr::StudentT::new(self.nu).unwrap()
    }
}

impl Distribution for StudentT {
    type Support = Reals;

    fn support(&self) -> Reals {
        Reals
    }

    fn cdf(&self, x: f64) -> Probability {
        use special_fun::FloatSpecial;

        let np1o2 = (self.nu + 1.0) / 2.0;
        let hyp2f1 = 0.5f64.hyp2f1(np1o2, 3.0 / 2.0, -x * x / self.nu);

        Probability::new_unchecked(
            0.5 + x * np1o2.gamma() * hyp2f1 / (self.nu * PI).sqrt() * (self.nu / 2.0).gamma()
        )
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        use rand_distr::Distribution;

        let sampler: rand_distr::StudentT<f64> = self.into();

        sampler.sample(rng)
    }
}

impl ContinuousDistribution for StudentT {
    fn pdf(&self, x: f64) -> f64 {
        use special_fun::FloatSpecial;

        let np1o2 = (self.nu + 1.0) / 2.0;
        let norm = np1o2.gamma() / (self.nu * PI).sqrt() / (self.nu / 2.0).gamma();

        norm * (1.0 + x * x / self.nu).powf(-np1o2)
    }
}

impl UnivariateMoments for StudentT {
    fn mean(&self) -> f64 {
        if self.nu <= 1.0 {
            unimplemented!("Mean is undefined for nu <= 1.");
        }

        0.0
    }

    fn variance(&self) -> f64 {
        if self.nu <= 1.0 {
            unimplemented!("Variance is undefined for nu <= 1.");
        } else if self.nu <= 2.0 {
            unimplemented!("Variance is infinite for 1 < nu <= 2.");
        }

        self.nu / (self.nu - 2.0)
    }

    fn skewness(&self) -> f64 {
        if self.nu <= 3.0 {
            unimplemented!("Skewness is undefined for nu <= 1.");
        }

        0.0
    }

    fn excess_kurtosis(&self) -> f64 {
        if self.nu <= 2.0 {
            unimplemented!("Kurtosis is undefined for nu <= 2.");
        } else if self.nu <= 4.0 {
            unimplemented!("Kurtosis is infinite for 2 < nu <= 4.");
        }

        6.0 / (self.nu - 4.0)
    }
}

impl Quantiles for StudentT {
    fn quantile(&self, _: Probability) -> f64 {
        unimplemented!()
    }

    fn median(&self) -> f64 {
        0.0
    }
}

impl Modes for StudentT {
    fn modes(&self) -> Vec<f64> {
        vec![0.0]
    }
}

impl Entropy for StudentT {
    fn entropy(&self) -> f64 {
        use special_fun::FloatSpecial;

        let no2 = self.nu / 2.0;
        let np1o2 = (self.nu + 1.0) / 2.0;

        np1o2 * (np1o2.gamma() - no2.gamma()) + (self.nu.sqrt() * (no2.beta(0.5)))
    }
}

impl fmt::Display for StudentT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StudentT({})", self.nu)
    }
}
