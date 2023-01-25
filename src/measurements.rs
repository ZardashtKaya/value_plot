use std::collections::VecDeque;

use egui::plot::{Value, Values};

pub struct Measurements{
    pub values: VecDeque<Value>,
    pub window_size: f64,
}

impl Measurements{
    pub fn new(window_size:f64) -> Self{
        Self{
            values: VecDeque::default(),
            window_size
        }
    }

    pub fn append_value(&mut self, v: Value){
        if let Some(last) = self.values.back(){
            if last.x >= v.x{
                self.values = VecDeque::default();
            }
        }

        let min_x = v.x - self.window_size;
        self.values.push_back(v);

        while let Some(value) = self.values.front(){
            if value.x < min_x {
                self.values.pop_front();

            } else {
                break;
            }
        }
    }

    pub fn get_values(&self)->Values{
        Values::from_values_iter(self.values.iter().copied())
        
    }

    pub fn append_str(&mut self, s: &str) {
        let parts = s.split(' ').collect::<Vec<&str>>();

        if parts.len() != 2 {
            return;
        }

        let x = parts.first().unwrap();
        let y = parts.last().unwrap();

        let x = match x.parse::<f64>(){
            Ok(value) => value,
            Err(_)=>return,
        };

        let y = match y.parse::<f64>(){
            Ok(value) => value,
            Err(_)=>return,
        };

        self.append_value(Value{x,y});
    }
}