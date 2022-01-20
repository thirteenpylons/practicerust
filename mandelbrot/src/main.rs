use num::Complex;


/// Try to determine if 'c' is in the Mandelbrot set, using at most ' limit'
/// iterations to decide.
///
/// If 'c' is not a member,return 'Some(i)', where 'i' is the number of
/// iterations it took for 'c' to leave the circle of radius 2 cernetered on the
/// origin. If 'c' seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that 'c' is not a member)
/// return 'None'

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    
    None
}

enum Option<T> {
    None,
    Some(T),
}

struct Complex {
    /// Real portion of the complex number;
    re: T,

    /// Imaginary portion of the complex number;
    im: T,
}

fn main() {
    println!("Hello, world!");
}
