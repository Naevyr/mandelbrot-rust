
use crate::complex_numbers::ComplexNumber;


pub(crate) fn PixelToComplex( x :i32,y:i32, scale : f64) -> ComplexNumber{
    ComplexNumber{a : (x as f64 / 200f64  -3f64)*scale,b : (y as f64 / 150f64 - 2f64)* scale}
}   

pub(crate) fn Mandel(coord: &ComplexNumber, n : u8) -> u8{
    
    let c = coord.clone();
    let mut x = ComplexNumber{a:0f64,b:0f64};
    for i in 0..n{

        if x.length() < 2f64{
            x = ComplexNumber::add(&x.pow2(),&c);
        }else {
            
            return i;
        }
        
    }
    n

}