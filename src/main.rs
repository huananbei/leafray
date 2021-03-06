extern crate rand; // https://github.com/rust-lang-nursery/rand
use rand::Rng;
use std::f64::consts::PI;

// const LEAFR: f64 = 0.05; // leaf radius [meters]

fn main() {
    let mut rng: rand::ThreadRng = rand::thread_rng();

    let mybox = Box {
        xmin: 0.0, xmax: 1.0,
        ymin: 0.0, ymax: 1.0,
        zmin: 0.0, zmax: 0.5,
    };

    let leaves = random_leaf_list(5, &mybox, &mut rng);
    print!("\n {:?}\n\n", leaves);

}

#[derive(Debug)]
struct Point { x: f64, y: f64, z: f64 }

#[derive(Debug)]
struct Normal { x: f64, y: f64, z: f64 }

#[derive(Debug)]
struct Leaf { p: Point, n: Normal }

#[derive(Debug)]
struct Box {
    xmin: f64, xmax: f64,
    ymin: f64, ymax: f64,
    zmin: f64, zmax: f64
}

fn random_leaf(b: &Box, rng: &mut rand::ThreadRng) -> Leaf {
    let p = Point {
        x: rng.gen_range(b.xmin,b.xmax),
        y: rng.gen_range(b.ymin,b.ymax),
        z: rng.gen_range(b.zmin,b.zmax),
    };
    // Random point in upper hemisphere in spherical coordinates
    let u: f64 = rng.gen();
    let v: f64 = rng.gen();
    let (theta, phi) = (v.acos(), 2. * PI * u);
    let n = Normal {
        x: theta.sin() * phi.cos(),
        y: theta.sin() * phi.sin(),
        z: theta.cos()
    };
    Leaf {p: p, n: n}
}

fn random_leaf_list(n: u64, b: &Box, rng: &mut rand::ThreadRng) -> Vec<Leaf> {
    let mut leaves: Vec<Leaf> = Vec::new();
    for _ in 0..n {
        leaves.push(random_leaf(&b, rng));
    }
    leaves
}
