use convolve2d::convolve2d;
use convolve2d::kernel;
use convolve2d::DynamicMatrix;
use rand::prelude::*;
use std::time::Instant;

fn main() {
    let a = DynamicMatrix::new(
        8,
        8,
        vec![
            0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //
        ],
    )
    .unwrap();
    println!("{}", format!("{a:?}"));

    let lapl_kernel = kernel::laplacian::full::<f32>();
    println!("{}", format!("{lapl_kernel:?}"));

    let output = convolve2d(&a, &lapl_kernel);
    println!("{}", format!("{output:?}"));

    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    for i in 1..20 {
        let now = Instant::now();
        let size = (2_usize).pow(i);
        println!("size: {}", size);
        let bv: Vec<f32> = (0..(size * size)).map(|_| rng.gen()).collect();
        let b = DynamicMatrix::new(size, size, bv).unwrap();
        println!("start");
        let output = convolve2d(&b, &lapl_kernel);
        println!("done {}s", now.elapsed().as_secs_f32());
    }
}
