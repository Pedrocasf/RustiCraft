use psp::{
    sys::{
        sceKernelLibcClock,
        SceKernelUtilsMt19937Context, 
        sceKernelUtilsMt19937Init, 
        sceKernelUtilsMt19937UInt
    },
    dprintln,
};
use num_traits::float::Float;
use na::Vector2;
/*use micromath::{
    F32Ext,
    vector::{
        Vector2<f32>,
        Vector2<i32>,
    },
};*/
use lazy_static::lazy_static;
use core::{mem::transmute, panicking::panic};
use spin::Mutex;
lazy_static!{
    static ref RAND_CTX:Mutex<SceKernelUtilsMt19937Context> = Mutex::new(unsafe{transmute([0;625])});
}
pub unsafe fn seed_context(){
    let seed = sceKernelLibcClock();
    let code = sceKernelUtilsMt19937Init(&mut *RAND_CTX.lock(), seed);
    dprintln!("{}",code);
    if code < 0{
        panic("error on seeding PRNG");
    };
}
pub fn rand()->u32{
    unsafe{sceKernelUtilsMt19937UInt(&mut *RAND_CTX.lock())}
}
fn interpolate(a0:f32,a1:f32,w:f32)->f32{
    return (a1 - a0) * w + a0;
}
fn random_gradient(i:Vector2<i32>)->Vector2<f32>{
    let rand = f32::from_bits(rand());
    return Vector2::<f32>::new((i.x as f32*rand).cos(),(i.y as f32*rand).sin());
}
fn dot_grid_gradient(i:Vector2<i32>, f:Vector2<f32>)->f32{
    let gradient = random_gradient(i);
    let dx = f.x - i.x as f32;
    let dy = f.x - i.y as f32;
    (dx*gradient.x) + (dy*gradient.y)
}
pub fn perlin(c:Vector2<f32>)->f32{
    let x0 = c.x as i32;
    let x1 = x0 + 1;
    let y0 = c.y as i32;
    let y1 = y0 + 1;

    let sx = c.x - x0 as f32;
    let sy = c.y - y0 as f32;

    let n0 = dot_grid_gradient(Vector2::<i32>::new(x0,y0), c);
    let n1 = dot_grid_gradient(Vector2::<i32>::new(x1,y0), c);
    let ix0 = interpolate(n0,n1,sx);

    let n0 = dot_grid_gradient(Vector2::<i32>::new(x0,y1), c);
    let n1 = dot_grid_gradient(Vector2::<i32>::new(x1,y1), c);
    let ix1 = interpolate(n0,n1,sx);
    
    interpolate(ix0, ix1, sy)
}
