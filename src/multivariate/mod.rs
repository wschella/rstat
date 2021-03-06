//! A collection of multivariate (i.e. multiple output) [distributions](trait.Distribution.html).
pub use crate::statistics::MultivariateMoments as Moments;

// Continuous:
pub mod normal;

pub mod lognormal;

pub mod dirichlet;

// Discrete:
pub mod multinomial;
