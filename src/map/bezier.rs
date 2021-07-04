#![feature(test)]

use rand::*;
use std::num::*;
use std::mem;

/*
  Here we calculate points of bezier curve only straightly
  So we use high degrees of the curve.
  @TODO: Add splitting curve into small number of bezier curves of n^3
  Note: This realization HAS bug. If we have a curve with very big curvature, at that point
  we'll have rather bad precision in curve points. ( Maybe add checking of tangent values? )
*/

#[derive(Default, Debug)]
pub struct Bezier {
    vertex_list: Vec<(f32, f32)>,
    interpolated_list: Vec<(f32, f32)>,
    interpolated_len: f32,
    interpolated: bool,
}

impl Bezier {
    pub fn new() -> Bezier {
        Bezier { interpolated: false, ..Default::default() }
    }

    pub fn clear(&mut self) {
        self.interpolated = false;
        self.vertex_list.clear();
        self.interpolated_list.clear();
        self.interpolated_len = 0.00;
    }

    pub fn add_vertexi(&mut self, point:(i32, i32)) {
        self.vertex_list.push( (point.0 as f32, point.1 as f32) );
    }

    pub fn add_vertexf(&mut self, point:(f32, f32)) {
        self.vertex_list.push(point);
    }

    pub fn get_tesselated_points(&mut self) -> Vec<(f32, f32)> {
        if !self.interpolated {
            self.interpolate();
        }
        self.interpolated_list.clone().to_owned()
    }

    fn interpolate(&mut self) {
        let step:f32 = 1.00 / self.tesselation_segments(self.approx_length()) as f32;
        let mut k:f32 = 0.00;
        let len = self.vertex_list.len();
        while k < 1.00 {
            let nk = 1.00 - k;
            let mut point = (0.0, 0.0);
            for p in 0..len {
                if p == 0 {
                    point.0 = nk.powi((len - 1) as i32) * self.vertex_list[p].0;
                    point.1 = nk.powi((len - 1) as i32) * self.vertex_list[p].1;
                } else if p == (len - 1) {
                    point.0 += k.powi((len - 1) as i32) * self.vertex_list[p].0;
                    point.1 += k.powi((len - 1) as i32) * self.vertex_list[p].1;
                } else {
                    point.0 += (len - 1) as f32 * nk.powi((len - (p+1)) as i32) * k.powi((p+1) as i32) * self.vertex_list[p].0;
                    point.1 += (len - 1) as f32 * nk.powi((len - (p+1)) as i32) * k.powi((p+1) as i32) * self.vertex_list[p].1;
                }
            }
            // println!("k={} point={:?}", k, point);
            self.interpolated_list.push(point);
            k += step;
        }

        self.interpolated_list.push(self.vertex_list[self.vertex_list.len()-1]);



        self.interpolated_len = 0.00;
        let mut p2:(f32, f32) = (0.0, 0.0);
        let mut iter = self.interpolated_list.iter();
        for i in 0..self.interpolated_list.len() {
            let p = self.interpolated_list[i];
            if i > 0 {
                self.interpolated_len += ((p.0 - p2.0)*(p.0 - p2.0) + (p.1 - p2.1)*(p.1 - p2.1)).sqrt();
            }
            p2 = p;
        }

        self.interpolated = true;
    }

    //Value at param k
    pub fn value(&self, k: f32) -> Result<(f32, f32), ()> {
        if self.vertex_list.len() < 3 {
            Err( () )
        } else {
            if k > 1.00 {
                Err( () )
            } else {
                Ok( self.interpolated_list[((self.interpolated_list.len() - 1) as f32 * k).round() as usize] )
            }
        }
    }

    pub fn approx_length(&self) -> f32 {
        let mut length = 0.00;
        let mut p2:(f32, f32) = (0.0, 0.0);
        let mut iter = self.vertex_list.iter();
        for i in 0..self.vertex_list.len() {
            let p = self.vertex_list[i];
            if i > 0 {
                length += ((p.0 - p2.0)*(p.0 - p2.0) + (p.1 - p2.1)*(p.1 - p2.1)).sqrt();
            }
            p2 = p;
        }
        length
    }

    //@TODO: Check if this formula produces enough points for our measures
    //it's min is 10 points, 40 for 500 length and 78 for 1000
    fn tesselation_segments(&self, len: f32) -> i32 {
        let min = 10.0;
        let segs = len / 10.0;
        (segs * segs * 0.6 + min * min).sqrt().ceil() as i32
    }
}

#[test]
fn spline_tesselate() {
    let mut spline = Bezier::new();

    spline.vertex_list.push( (0.0, 0.0) );
    spline.vertex_list.push( (0.0, 1.0) );
    spline.vertex_list.push( (1.0, 0.0) );
    spline.vertex_list.push( (1.0, 1.0) );

    let mut points = spline.get_tesselated_points();
    assert!(true)
}


#[cfg(test)]
mod tests {
    use super::*;
    use map::bezier::test::Bencher;

    #[bench]
    fn bench_interpolation(b: &mut Bencher) {
        b.iter(|| {
            let mut spline = super::Bezier::new();

            //here approx length for curve will be about 1000
            for i in 0..10 {
                spline.vertex_list.push( (i as f32*100.0 as f32, super::rand::random::<f32>() * 100.0) );
            }

            spline.get_tesselated_points();
        });
    }

}
