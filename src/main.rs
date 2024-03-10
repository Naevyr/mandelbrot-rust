extern crate sdl2;


use sdl2::gfx::primitives::DrawRenderer;

use sdl2::mouse::MouseButton;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


mod util;
use util::PixelToComplex;
mod complex_numbers;
use crate::complex_numbers::ComplexNumber as Complex;




fn Mandel(coord: &Complex, n : u8) -> u8{
    
    let c = coord.clone();
    let mut x = Complex{a:0f64,b:0f64};
    for i in 0..n{

        if x.length() < 2f64{
         
            x = Complex::add(&x.pow2(),&c);
        }else {
            
            return i;
        }
        
    }
    n
}



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
   
    
    

    let mut mouse_down : bool = false; 
    let mut offset : Complex = Complex{a:0f64,b:0f64};
    let mut scale : f64 = 1.0;

    let mut color = Color::RGB(0, 0, 0);
   
    'running: loop {
        

      
  

        
       
        for x  in 0..800 {
            for y in 0..600 {
                let c =Complex::add(& PixelToComplex(x as i32 ,y as i32,scale) ,& offset);
                
                
                let inside = Mandel(&c, 30);
                color.r = 255 / 30 * inside; 
                canvas.pixel(x , y , color);
               
            }
        }
        
        
       
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y }=>
                {
                    if mouse_btn == MouseButton::Right {mouse_down = true;}
                    
                },
                Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } =>
                {
                    if mouse_btn == MouseButton::Right {mouse_down = false;}
                },
                Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => 
                {
                    
                    if(mouse_down){

                        offset = Complex::sub(&offset,&Complex{a:xrel as f64 / 200f64 *scale,b:yrel as f64/ 150f64 *scale});
                        println!("{:}",offset);
                    }
                },
                
                Event::MouseWheel { timestamp, window_id, which, x, y, direction }=>{


                        scale = scale + y as f64 / 100.0 * scale;
                        
                    
                },
                
                
                
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
              
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
