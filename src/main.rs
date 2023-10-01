use perlin_2d::NoiseMap2D;
use rand::Rng;
mod visualize;
mod perlin_2d;




fn main() {

    let mut window = visualize::Window::new([500,500]);
    let mut octaves : Vec<[u64;2]> = Vec::new();
    let mut rng = rand::thread_rng();

    octaves.push([2,2]);
    octaves.push([10,10]);
    octaves.push([20,20]);
    octaves.push([200,200]);



    let mut map = NoiseMap2D::new(octaves);

    for _i in 1..100 {

        map.rotate(0.2);
    
        for y in 1..=500{

            for x in 1..=500{

                let value = map.poll(x as f64 / 500.0, y as f64 / 500.0);
                let (mut r,mut g,mut b) = ((255.0*value) as u8, (255.0*value) as u8, (255.0*value) as u8);

                r = (value * 255.0).floor() as u8;
                g = (value * 255.0).floor() as u8;
                b = (value * 255.0).floor() as u8;
                    
                
                let color = sdl2::pixels::Color::RGB(r, g, b);

                window.canvas.set_draw_color(color);
                window.canvas.draw_point(sdl2::rect::Point::new(x,y)).unwrap();
                
            

            }
            

        }

        window.canvas.present();

        
    }

    window.showcase();


}
