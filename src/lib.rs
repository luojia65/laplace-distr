use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::Float;
use rand_distr::Standard;

pub struct Laplace<N> {
    beta: N,
}

impl<N: Float> Laplace<N> {
    pub fn new(beta: N) -> Laplace<N> {
        Laplace { beta }
    }
}

impl<N: Float> Distribution<N> for Laplace<N>
where
    Standard: Distribution<N>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> N {
        let p = rng.gen();
        if rng.gen_bool(0.5) {
            -self.beta * N::ln(N::from(1.0) - p)
        } else {
            self.beta * N::ln(p)
        }
    }
}
