use sdl2;


pub struct Window {
    pub canvas : sdl2::render::WindowCanvas,
    pub resolution : [u32; 2],
    pub event_pump : sdl2::EventPump,
    running : bool,
}

impl Window {

    pub fn new( resolution : [u32; 2] ) -> Window {
        
        let context = sdl2::init().expect("Failed to init SDL, Check installation;");
        let video = context.video().expect("Failed to Initialize Video Subsystem for sdl.");
        let window = video.window("Noise Visualization",resolution[0], resolution[1])
        .position_centered()
        .build().expect("Failed to build Window.");
        let canvas = window.into_canvas().accelerated().build().expect("Failed to Create Canvas");
        let event_pump = context.event_pump().expect("Failed to Create Event Pump");

        Window {
            canvas : canvas,
            resolution : resolution,
            event_pump : event_pump,
            running : true,
        }

        
    }

    pub fn showcase(&mut self) {

        self.canvas.present();

        while self.running {

            for event in self.event_pump.poll_iter(){

                match event {
                    sdl2::event::Event::Quit { .. } => self.running = false,
                    _ => {},
                }
            }

        }

    }



}