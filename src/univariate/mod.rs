//! A collection of univariate (i.e. scalar output) [distributions](trait.Distribution.html).
pub use crate::statistics::UnivariateMoments as Moments;

#[inline]
pub(self) fn factorial(n: u64) -> u64 { (2..n).product() }

#[inline]
pub(self) fn choose(n: u64, k: u64) -> u64 {
    (1..k).fold(n, |acc, i| acc * (n - i)) / factorial(k)
}

// Miscellaneous:
pub mod uniform;

pub mod degenerate;

// Continuous:
pub mod arcsine;

pub mod beta;

pub mod beta_prime;

pub mod cauchy;

pub mod chi;

pub mod chi_sq;

pub mod cosine;

pub mod erlang;

pub mod exponential;

pub mod f_dist;

pub mod folded_normal;

pub mod frechet;

pub mod gamma;

pub mod gev;

pub mod gpd;

pub mod gumbel;

pub mod inverse_gamma;

pub mod inverse_normal;

pub mod kumaraswamy;

pub mod laplace;

pub mod levy;

pub mod logistic;

pub mod lognormal;

pub mod normal;

pub mod pareto;

pub mod rayleigh;

pub mod student_t;

pub mod triangular;

pub mod weibull;

// Discrete:
pub mod bernoulli;

pub mod beta_binomial;

pub mod binomial;

pub mod categorical;

pub mod geometric;

pub mod poisson;
