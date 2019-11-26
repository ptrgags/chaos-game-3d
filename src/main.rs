extern crate rand;
use rand::Rng;

type Vector = (f32, f32, f32);

trait Transform {
    fn transform(&self, vector: &Vector) -> Vector;
}

struct SierpinskiTransform {
    translation: Vector
}

impl SierpinskiTransform {
    pub fn new(translation: Vector) -> SierpinskiTransform {
        SierpinskiTransform { translation }
    }
}

impl Transform for SierpinskiTransform {
    fn transform(&self, vector: &Vector) -> Vector {
        let (x, y, z) = vector;
        let (tx, ty, tz) = self.translation;
        let x2 = 0.5 * x + tx;
        let y2 = 0.5 * y + ty;
        let z2 = 0.5 * z + tz;

        (x2, y2, z2)
    }
}


fn print_point(vector: &Vector, index: u32) {
    const SCALE: f32 = 10000000.0;
    let (x, y, z) = vector;
    let shade = index % 256;
    let shade2 = (index + 64) % 256;
    println!("{},{},{},{},{},{}", x * SCALE, y * SCALE, z * SCALE, 0, shade, shade2);
}

fn main() {
    let functions: [SierpinskiTransform; 4] = [
        SierpinskiTransform::new((-0.5, -0.5, 0.0)),
        SierpinskiTransform::new((0.5, -0.5, 0.0)),
        SierpinskiTransform::new((0.0, 0.5, -0.5)),
        SierpinskiTransform::new((0.0, 0.5, 0.5))
    ];

    const DISCARD_ITERS: u32 = 5;
    const TOTAL_ITERS: u32 = 1000000;
    let mut rng = rand::thread_rng();

    let start_point: Vector = (
        rng.gen_range(-1.0f32, 1.0f32),
        rng.gen_range(-1.0f32, 1.0f32),
        rng.gen_range(-1.0f32, 1.0f32)
    );

    let mut current_point = start_point;
    for i in 0..TOTAL_ITERS {
        if i < DISCARD_ITERS {
            continue;
        }

        print_point(&current_point, i);
        let index = rng.gen_range(0usize, 4usize);
        current_point = functions[index].transform(&current_point);
    }
}
