use std::default;

use std::mem;
use crate::map::bezier::Bezier;

pub const MAX_X:i32 = 512;
pub const MAX_Y:i32 = 384;

#[derive(PartialEq, Debug)]
pub enum HitObjectType {
    Circle,
    Spinner,
	Slider,
}

impl Default for HitObjectType {
    fn default() -> HitObjectType { HitObjectType::Circle }
}

#[derive(Debug)]
pub enum SliderType {
    SliderNone,
    SliderCatmul,
	SliderBezier,
	SliderLinear,
	SliderPassthrough
}

impl Default for SliderType {
    fn default() -> SliderType { SliderType::SliderNone }
}

#[derive(Debug)]
pub enum HitSoundType {
    Normal,
    Whistle,
    Finish,
    Clap
}

impl Default for HitSoundType {
    fn default() -> HitSoundType { HitSoundType::Normal }
}

//Note: Maybe i need to use generics?
#[derive(Default, Debug)]
pub struct HitObject {
    pub obj_type: HitObjectType,
    pub points: Vec<(i32, i32)>,
    pub time_start: i32,
    pub length: i32,
    pub end_combo: bool,
    pub combo_index: i32,
    pub combo_number: i32,
    pub slider_type: SliderType,
    pub slider_repeats: i32,
    pub slider_curve_points: Vec<(f32, f32)>,

    pub hitsound: HitSoundType
}

impl HitObject {
    pub fn new(obj_type: HitObjectType) -> HitObject {
        HitObject { obj_type: obj_type, combo_number: 1, ..Default::default() }
    }

    pub fn with_params(&mut self, time_start: i32, end_combo: bool, hitsound: HitSoundType) {
        self.time_start = time_start;
        self.end_combo = end_combo;
        self.hitsound = hitsound;
    }

    pub fn with_spinner_length(&mut self, length: i32) {
        if self.obj_type == HitObjectType::Spinner {
            self.length = length;
        }
    }

    pub fn add_point(&mut self, point: (i32, i32)) {
        if (self.obj_type != HitObjectType::Slider) && (self.points.len() > 0) {}
        else {
            self.points.push(point);
        }
    }

    pub fn prepare_slider(&mut self) {
        if self.obj_type == HitObjectType::Slider {
            match self.slider_type {
                SliderType::SliderNone => {}
                SliderType::SliderLinear | SliderType::SliderPassthrough => {
                    let mut cl:Vec<(i32, i32)> = self.points.clone();
                    for i in 0..cl.len() {
                        self.slider_curve_points.push( (cl[i].0 as f32, cl[i].1 as f32) );
                    }
                }
                SliderType::SliderBezier | SliderType::SliderCatmul => {
                    let mut curve = Bezier::new();
                    for i in 0..self.points.len() {
                        curve.add_vertexi(self.points[i]);
                    }
                    self.slider_curve_points = curve.get_tesselated_points();
                }
            }
        }
    }

    pub fn get_time_start(&self) -> i32 {
        self.time_start
    }
}
