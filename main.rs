extern crate ndarray;
use ndarray::prelude::*;
//[dependencies]
//ndarray = { version = "0.13.0", features = ["blas"] }


fn hello() {
    let a = Array::<f64, _>::zeros(3);
    println!("{}",a);

    let b = Array::range(0.,9.,1.).into_shape((3,3)).unwrap();
    println!("{}",b);

    println!("Hello, world!");
}

fn heat() {
    let L = 1.0;
    let M = 10;
    
    let dx = L/M as f64;//f64 float

    let dt = 0.01;
    let N = 500;

    let alpha = 0.01;
    let gamma = dt/(dx*dx) as f64;

    let a = alpha*gamma;

    let mut T = Array::<f64, _>::zeros(M);
    for i in 0..M {
        T[i] = 40.0;
    }
    println!("{}",T);

    let mut T_n = Array::<f64, _>::zeros(M);//next T

    
    for step in 0..N {
        for i in 0..M {
            let mut T_b = if i == 0 { 10.0 } else { T[i-1] };
            let mut T_f = if i == M-1 { T[M-1] } else { T[i+1] };
            T_n[i] = a*T_f+(1.0-2.0*a)*T[i]+a*T_b;
        }
        T = T_n.clone();
        if step%10 == 0 {
            println!("{}",T);
        }
    }

}

fn main() {
    hello();
    heat();
}
