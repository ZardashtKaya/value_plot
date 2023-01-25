mod measurements;
use std::{thread, io::{BufRead}, sync::{Arc, Mutex}};
use clap::{Parser};
use eframe::{epi, NativeOptions};
use egui::plot::{Plot, Line};
use measurements::Measurements;

struct App {
    measurements:Arc<Mutex<Measurements>>,
    include_y:Vec<f64>

}


impl App {
    fn new(window_size: f64,include_y: Vec<f64>) -> Self {
        Self {
            measurements:Arc::new(Mutex::new(
                Measurements::new(window_size)
            )),
            include_y
        }
    }
}

impl epi::App for App {
    
    

    fn name(&self) -> &str {
        "Monitoring App"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            let mut plot = Plot::new("measurements");
            
            for &y in self.include_y.iter(){
                plot = plot.include_y(y)
            }
            
            plot.show(ui, |plot_ui|{
                plot_ui.line(Line::new(self.measurements.lock().unwrap().get_values())); 
                
            });
        });
        ctx.request_repaint();
    } 
    

    

}


#[derive(Parser,Debug)]
#[clap(author,version,about,long_about = None)]
    pub struct Args{
        /// window size in microseconds
        #[clap(short,long,default_value_t=4000.0)]
        pub window:f64,
        /// Y height
        #[clap(short,long)]
        pub include_y:Vec<f64>,
    
    }


fn main() {

    let args = Args::parse();

    let  app = App::new(args.window,args.include_y);

    let io_measurements = app.measurements.clone();

    thread::spawn(move ||{
        let stdin = std::io::stdin();
        for line in stdin.lock().lines(){
            match line{
                Ok(s)=>{
                    io_measurements.lock().unwrap().append_str(s.as_str());
                },
                _ =>return
            }
        }
    });
    let native_options = NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);

}
