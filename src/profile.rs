#![allow(dead_code)]

use std::time;
use crate::widget::textbox::TextBox;
use crate::graphic::Graphic;
use crate::color::Color;
use crate::global::GLOBAL_ENV;

pub struct Profiler {
    text_count: usize,
    image_count: usize,
    graphic_count: usize,
    frame_count: usize,
    start_time: time::Instant,
    last_ticks: Vec<time::Instant>,
    pub rendered: Vec<Graphic>  // rendered data of the last frame
}

impl Profiler {

    pub fn new() -> Self {
        Profiler {
            text_count: 0,
            image_count: 0,
            graphic_count: 0,
            frame_count: 0,
            start_time: time::Instant::now(),
            last_ticks: vec![time::Instant::now();64],
            rendered: vec![]
        }
    }

    pub unsafe fn init_function_profilers() {
        FUNCTION_PROFILERS = vec![FunctionProfiler::new();16];
    }

    pub fn count_graphic(&mut self, graphic: &Graphic) {

        match graphic {
            Graphic::Text { .. } => {
                self.text_count += 1;
            }
            Graphic::Image { .. } => {
                self.image_count += 1;
            },
            Graphic::RoundRect { thickness, .. } => {

                if thickness > &0.0 {
                    self.graphic_count += 20;
                }

                else {
                    self.graphic_count += 19;
                }

            },
            Graphic::Polygon { points, .. } => {
                self.graphic_count += points.len() + 1;
            },
            _ => {
                self.graphic_count += 1;
            }
        }

    }

    pub fn new_frame(&mut self, tick: time::Instant) {
        self.render_frame();

        self.text_count = 0;
        self.image_count = 0;
        self.graphic_count = 0;
        self.frame_count += 1;

        self.last_ticks[self.frame_count % 64] = tick;
    }

    pub fn calc_fps(&self) -> f32 {

        if self.frame_count < 64 {
            0.0
        }

        else {
            let frames_64 = self.last_ticks[self.frame_count % 64].duration_since(self.last_ticks[(self.frame_count + 1) % 64]).as_micros() as f32;
            1_000_000.0 / (frames_64 / 64.0)
        }

    }

    fn render_frame(&mut self) {
        let info = format!(
            "elapsed time: {}ms\nframe: {}\nfps: {}\ntext per frame: {}\nimage per frame: {}\ngraphic per frame: {}",
            time::Instant::now().duration_since(self.start_time.clone()).as_millis(),
            self.frame_count,
            self.calc_fps(),
            self.text_count,
            self.image_count,
            self.graphic_count
        );
        let mut text = TextBox::new(&info, 15.0, 15.0, 240.0, 240.0, 18.0).set_color(Color::new(255, 255, 255, 255)).to_owned();

        self.rendered = vec![
            vec![Graphic::new_rect(0.0, 0.0, 270.0, 270.0, 0.0, Color::new(128, 128, 128, 128))],
            text.render()
        ].concat();
    }
}

#[derive(Clone)]
struct FunctionProfiler {
    called: usize,
    returned: usize,
    time_stack: Vec<time::Instant>,
    time_nano: u128
}

impl FunctionProfiler {

    pub fn new() -> Self {
        FunctionProfiler {
            called: 0,
            returned: 0,
            time_stack: vec![],
            time_nano: 0
        }
    }

    pub fn calc_avg(&self) -> f64 {

        if self.returned > 0 {
            self.time_nano as f64 / self.returned as f64
        }

        else {
            0.0
        }

    }

}

// I'm not sure if it'd work on multi-threaded environments
static mut FUNCTION_PROFILERS: Vec<FunctionProfiler> = vec![];

// `fn_start` and `fn_end` are unsafe functions, but I'm not declaring them as unsafe,
// for easy profiling
fn fn_start(index: usize) {

    let start_time = time::Instant::now();

    unsafe {
        FUNCTION_PROFILERS[index].called += 1;
        FUNCTION_PROFILERS[index].time_stack.push(start_time);
    }

}

fn fn_end(index: usize) {

    let end_time = time::Instant::now();

    unsafe {
        FUNCTION_PROFILERS[index].returned += 1;

        match FUNCTION_PROFILERS[index].time_stack.pop() {
            None => {
                GLOBAL_ENV.raise_error("Something went wrong with the function-profilers!".to_string());
            }
            Some(start_time) => {
                FUNCTION_PROFILERS[index].time_nano += end_time.duration_since(start_time.clone()).as_nanos();
            }
        }

    }

}
