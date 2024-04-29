mod simulation;
use simulation::{Body, simulate_step};

fn main() {
    let mut bodies: Vec<Body<2>> = vec![
        Body::new(1000000000.0, [0.0, -1.0], [0.0,-0.1]),
        Body::new(1000000000.0, [0.0, 1.0], [0.0,0.1]),
    ];

    for _ in 0..100000000 {
        println!("Body 1: {:?} | Body 2: {:?}", bodies[0].position, bodies[1].position);
        bodies = simulate_step(bodies, 0.0001);
    }
}
