
use std::{fmt, f32::consts::PI, ops};    
pub struct ComplexNumber
{
    pub a : f64,
    pub b : f64
}



impl ComplexNumber {
   
    pub (crate) fn pow2(&self) -> ComplexNumber {
        ComplexNumber::mul(self,self) 

    }
    
    pub (crate) fn powf(&self, t : i32) -> ComplexNumber {
        
        let r : f64 = self.length();
        let th : f64 = self.angle();
        let p = f64::powi(r, t); 


        

        ComplexNumber{ a : p *  f64::cos(t as f64 * th) , b : p  * f64::sin(  t as f64 * th)}

    }
    

    pub(crate) fn length(&self) -> f64 {
        
        f64::powf(f64::powi(self.a, 2) + f64::powi(self.b, 2),0.5)

    }

    fn angle(&self) -> f64{
        if f64::abs(self.a) > f64::EPSILON {
        
        
            if self.a > 0f64 {
                f64::atan(self.b/self.a)
            } 
            else if self.a < 0f64 && self.b > 0f64{
                f64::atan(self.b/self.a) + PI as f64
            } 
            else {
                f64::atan(self.b/self.a) - PI as f64
            }
        }else {
            0f64
        }
    
           
        
        
    }

    pub fn add(a: &ComplexNumber, b: &ComplexNumber) -> ComplexNumber {
        
        ComplexNumber { a: a.a + b.a, b: a.b + b.b }
    }
    pub fn sub(a: &ComplexNumber, b: &ComplexNumber) -> ComplexNumber {
        
        ComplexNumber { a: a.a - b.a, b: a.b - b.b }
    }

    pub fn mul(a: &ComplexNumber, b: &ComplexNumber) -> ComplexNumber{
        ComplexNumber { a: a.a * b.a - a.b * b.b, b: a.a * b.b + a.b * b.a }
    }


    pub (crate) fn clone(&self) -> ComplexNumber {
        ComplexNumber { a: self.a, b: self.b }
    }

}





impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}



