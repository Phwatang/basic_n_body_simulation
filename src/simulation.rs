use core::time;
/// Code for running the n_body calculations
use std::iter;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Neg;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector<const N: usize> {
    data: [f64; N],
}
impl<const N: usize> Vector<N> {
    pub fn new(data: [f64; N]) -> Self {
        Self{ data }
    }

    pub fn dist_from(&self, other: &Self) -> f64 {
        let sum_squared_diffs = self.data.iter().zip(other.data.iter())
            .map(|(&x1, &x2)| (x1 - x2).powi(2))
            .sum();
        return f64::sqrt(sum_squared_diffs);
    }

    pub fn normalise(&mut self) {
        let sum_of_squares = self.data.iter().map(|x| x.powi(2)).sum();
        let norm_factor = 1.0 / f64::sqrt(sum_of_squares);
        *self *= norm_factor;
    }
}
impl<const N: usize> AddAssign for Vector<N> {
    fn add_assign(&mut self, other: Self) {
        self.data.iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, &b)| *a += b);
    }
}
impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        return self;
    }
}
impl<const N: usize> SubAssign for Vector<N> {
    fn sub_assign(&mut self, other: Self) {
        self.data.iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, &b)| *a -= b);
    }
}
impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        return self;
    }
}
impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        return self;
    }
}
impl<const N: usize> MulAssign<f64> for Vector<N> {
    fn mul_assign(&mut self, rhs: f64) {
        self.data.iter_mut()
            .for_each(|a| *a *= rhs);
    }
}
impl<const N: usize> Neg for Vector<N> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self *= -1.0;
        return self;
    }
}


/// Data relevant for a single "body" in the simulation
#[derive(Debug, Clone)]
pub struct Body<const DIM: usize> {
    pub mass: f64,
    pub position: Vector<DIM>,
    pub velocity: Vector<DIM>,
}
impl<const DIM: usize> Body<DIM> {
    pub fn new(mass: f64, position: [f64; DIM], velocity: [f64; DIM]) -> Self {
        Self {mass, position: Vector::<DIM>::new(position), velocity: Vector::<DIM>::new(velocity)}
    }
}

const GRAVITY_CONST: f64 = 0.00000_00000_66743;

/// Updates the data of the bodies given using Newton's law of gravitation
pub fn simulate_step<const DIM: usize>(mut bodies: Vec<Body<DIM>>, time_res: f64) -> Vec<Body<DIM>> 
{
    let n = bodies.len();
    // Update velocities
    (0..n-1).flat_map(
        |x| (x+1..n).map(move |y| (y, x))
    ).for_each(|(i, j)| {
        // Calculate force direction
        let mut force = bodies[j].position - bodies[i].position;
        force.normalise();
        // Calculate force strength
        let force_mag = GRAVITY_CONST * bodies[j].mass * bodies[i].mass * (1.0 / bodies[j].position.dist_from(&bodies[i].position).powi(2));
        force *= force_mag;
        // Apply time resolution
        force *= time_res;
        // Apply force to velocities
        let mass_i = bodies[i].mass;
        let mass_j = bodies[j].mass;
        bodies[i].velocity += force * (1.0 / mass_i);
        bodies[j].velocity -= force * (1.0 / mass_j);
    });

    // Update positions
    bodies.iter_mut().for_each(|b| {
        b.position += b.velocity * time_res;
    });

    return bodies;
}
