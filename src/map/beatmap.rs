#![feature(test)]

use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::str::Lines;
// use hit_object::*;
use crate::map::hit_object::*;

#[derive(Debug, Copy, Clone)]
pub enum SampleSet {
    NONE,
    NORMAL,
    SOFT
}

impl Default for SampleSet {
    fn default() -> SampleSet { SampleSet::NONE }
}

#[derive(Debug, Copy, Clone)]
enum SectionType {
    General,
    Editor,
    Metadata,
    Difficulty,
    Events,
    TimingPoints,
    Colours,
    HitObjects
}

#[derive(Debug, Copy, Clone, Default)]
pub struct TimingPoint {
    inherited: bool,

    time_start: i32,
    //Length of beat in ms for non-inherited
    beat_length: f32,
    //slider multiplier for inherited
    velocity: i32,
    //beats per measure
    meter: i32,

    sample_type: SampleSet,
    sample_type_custom: i32,
    //sample volume 0 to 100
    sample_volume: i32,

    //kiai mode
    kiai: bool
}

impl TimingPoint {
    pub fn get_slider_multiplier(&self) -> f32 {
        self.velocity as f32 / -100.0
    }
}

#[derive(Default, Debug)]
pub struct BeatmapStat {
    circles: i32,
    sliders: i32,
    spinners: i32,
    min_bpm: i32,
    max_bpm: i32
}

pub trait FileFormat {
    fn from_osu(&mut self, lines: &mut Vec<&str>);
}

/*
    @TODO: add support for Events section
*/
#[derive(Default, Debug)]
pub struct Beatmap {
    //Main data
    id: i32,
    set_id: i32,
    objects: Vec<HitObject>,
    timing_points: Vec<TimingPoint>,

    //general data
    audio_filename: String,

    //parameters
    stack_leniency: f32,
    beatmap_set_id: i32,
    letterbox_in_breaks: bool,
    widescreen_storyboard: bool,
    epilepsy_warning: bool,

    audio_lead_in: i32,
    preview_time: i32,
    countdown_type: i32,
    sample_set: SampleSet,

    //difficulty
    hp_drain: f32,
    circle_size: f32,
    overall_difficulty: f32,
    approach_rate: f32,
    slider_multiplier: f32,
    slider_tick_rate: f32,

    //meta data
    title: String,
    title_unicode: String,
    artist: String,
    artist_unicode: String,
    creator: String,
    difficulty: String,
    source: String,
    tags: String,

    //RGBA format
    colors: Vec<(i32, i32, i32, f32)>,

    stat: BeatmapStat,


    //editor related
}

impl Beatmap {
    pub fn new() -> Self {
        Beatmap { ..Default::default() }
    }
}

impl FileFormat for Beatmap {
    fn from_osu(&mut self, lines: &mut Vec<&str>) {
        let mut section = SectionType::General;
        for _line in lines {
            let mut line = _line.trim().to_string();
            match line.as_ref() {
                //setting up mode
                "[General]" => { section = SectionType::General }
                "[Editor]" => { section = SectionType::Editor }
                "[Metadata]" => { section = SectionType::Metadata }
                "[Difficulty]" => { section = SectionType::Difficulty }
                "[Events]" => { section = SectionType::Events }
                "[TimingPoints]" => { section = SectionType::TimingPoints }
                "[Colours]" => { section = SectionType::Colours }
                "[HitObjects]" => { section = SectionType::HitObjects }
                _ => {
                    //parsing here
                    match section {
                        SectionType::General => {
                            let mut tokens:Vec<&str> = line.split(':').collect();
                            if tokens.len() > 1 {
                                tokens[1] = tokens[1].trim();
                            }
                            match tokens[0] {
                                "AudioFilename" => { self.audio_filename = tokens[1].to_string() }
                                "AudioLeadIn" => { self.audio_lead_in = tokens[1].parse().unwrap() }
                                "PreviewTime" => { self.preview_time = tokens[1].parse().unwrap() }
                                "Countdown" => { self.countdown_type = tokens[1].parse().unwrap() }
                                "SampleSet" => {
                                    self.sample_set = match tokens[1] {
                                        "Normal" => { SampleSet::NORMAL }
                                        "Soft" => { SampleSet::SOFT }
                                        "None" => { SampleSet::NONE }
                                        _ => { SampleSet::NONE }
                                    }
                                }
                                "StackLeniency" => { self.stack_leniency = tokens[1].parse().unwrap() }
                                "LetterboxInBreaks" => {
                                    let val:i32 = tokens[1].parse().unwrap();
                                    if val == 1 {
                                        self.letterbox_in_breaks = true;
                                    }
                                }
                                _ => {}
                            }
                        }
                        SectionType::Metadata => {
                            let mut tokens:Vec<&str> = line.split(':').collect();
                            if tokens.len() > 1 {
                                tokens[1] = tokens[1].trim();
                            }
                            match tokens[0] {
                                "Title" => { self.title = tokens[1].to_string() }
                                "Artist" => { self.artist = tokens[1].to_string() }
                                "Creator" => { self.creator = tokens[1].to_string() }
                                "Version" => { self.difficulty = tokens[1].to_string() }
                                "Source" => { self.source = tokens[1].to_string() }
                                "Tags" => { self.tags = tokens[1].to_string() },
                                "BeatmapID" => { self.id = tokens[1].parse().unwrap() }
                                "BeatmapSetID" => { self.set_id = tokens[1].parse().unwrap() }
                                _ => {}
                            }
                        }
                        SectionType::Difficulty => {
                            let mut tokens:Vec<&str> = line.split(':').collect();
                            if tokens.len() > 1 {
                                tokens[1] = tokens[1].trim();
                            }
                            match tokens[0] {
                                "HPDrainRate" => { self.hp_drain = tokens[1].parse().unwrap() }
                                "CircleSize" => { self.circle_size = tokens[1].parse().unwrap() }
                                "OverallDifficulty" => { self.overall_difficulty = tokens[1].parse().unwrap() }
                                "ApproachRate" => { self.approach_rate = tokens[1].parse().unwrap() }
                                "SliderMultiplier" => { self.slider_multiplier = tokens[1].parse().unwrap() }
                                "SliderTickRate" => { self.slider_tick_rate = tokens[1].parse().unwrap() }
                                _ => {}
                            }
                        }
                        SectionType::TimingPoints => {
                            let mut tokens:Vec<&str> = line.split(',').collect();
                            if tokens.len() > 6 {
                                let mut timing = TimingPoint { ..Default::default() };
                                let time_start:f32 = tokens[0].parse().unwrap();
                                timing.time_start = (time_start.round()) as i32;
                                timing.meter = tokens[2].parse().unwrap();
                                timing.sample_type = match tokens[3].parse().unwrap() {
                                    1 => { SampleSet::NORMAL }
                                    2 => { SampleSet::SOFT }
                                    _ => { SampleSet::NONE }
                                };

                                timing.sample_type_custom = tokens[4].parse().unwrap();
                                timing.sample_volume = tokens[5].parse().unwrap();
                                let kiai:i8 = tokens[7].parse().unwrap();
                                timing.kiai = kiai > 0;
                                let vel:f32 = tokens[1].parse().unwrap();
                                if vel.is_sign_negative() {
                                    timing.inherited = true;
                                    timing.velocity = (vel.round()) as i32;
                                } else {
                                    timing.inherited = false;
                                    timing.beat_length = vel;
                                }
                                self.timing_points.push(timing);
                            }
                        }
                        SectionType::HitObjects => {
                            let tokens:Vec<&str> = line.split(',').collect();
                            if tokens.len() > 5 {
                                let hit_type:i32 = tokens[3].parse().unwrap();
                                let time_start:i32 = tokens[2].parse().unwrap();
                                let mut obj = HitObject::new(HitObjectType::Circle);
                                obj.with_params(
                                    time_start,
                                    false,
                                    match tokens[4].parse().unwrap() {
                                        0 => { HitSoundType::Normal }
                                        1 => { HitSoundType::Whistle }
                                        2 => { HitSoundType::Finish }
                                        3 => { HitSoundType::Clap }
                                        _ => { HitSoundType::Normal }
                                    }
                                );
                                obj.add_point( (
                                    tokens[0].parse().unwrap(),
                                    tokens[1].parse().unwrap()
                                ) );
                                match hit_type {
                                    12 => {
                                        //Spinner
                                        obj.obj_type = HitObjectType::Spinner;
                                        let mut time_end: i32 = tokens[5].parse().unwrap();
                                        time_end = time_end - obj.time_start;
                                        obj.with_spinner_length( time_end );
                                    }
                                    2 | 6 => {
                                        //Slider
                                        obj.obj_type = HitObjectType::Slider;
                                        obj.slider_repeats = tokens[6].parse().unwrap();
                                        let slider_parts:Vec<&str> = tokens[5].split('|').collect();
                                        match slider_parts[0] {
                                            "B" | "C" => {
                                                //Catmul was marked as deprecated and isnt used right now.
                                                //Maybe i should add it?
                                                obj.slider_type = SliderType::SliderBezier;
                                            }
                                            "P" => {
                                                obj.slider_type = SliderType::SliderPassthrough;
                                            }
                                            "L" => {
                                                obj.slider_type = SliderType::SliderLinear;
                                            }
                                            _ => {}
                                        }
                                        for i in 1..slider_parts.len() {
                                            let point:Vec<&str> = slider_parts[i].split(':').collect();
                                            obj.add_point( (point[0].parse().unwrap(), point[1].parse().unwrap()) );
                                        }
                                        obj.prepare_slider();
                                    }
                                    _ => {}
                                }

                                self.objects.push(obj);
                            }
                        }
                        SectionType::Events => { /* @TODO: implement */ }
                        SectionType::Colours => {
                            let mut tokens:Vec<&str> = line.split(':').collect();
                            if tokens.len() > 1 {
                                tokens[1] = tokens[1].trim();
                                let mut color_frags:Vec<&str> = tokens[1].split(',').collect();
                                let mut color = (0,0,0,1.0);
                                color.0 = color_frags[0].parse().unwrap();
                                color.1 = color_frags[1].parse().unwrap();
                                color.2 = color_frags[2].parse().unwrap();
                                self.colors.push(color);
                            }
                        }
                        SectionType::Editor => {}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn read_map() {
        let mut btmp = super::Beatmap::new();
        let mut f = File::open("./bin/test.osu").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s);
        let mut lines = s.lines().collect();
        btmp.from_osu(&mut lines);
        println!("{:?}", btmp);

        assert!(true)
    }
}
