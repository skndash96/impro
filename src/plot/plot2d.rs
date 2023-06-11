use crate::ops::CH_LEN;

use super::canvas::Canvas;
use super::{ Pos, Res, Pixel };

use super::node::{
    Rect, EqnQuad1
};

pub struct Plot2D<'a> {
    cv: Option<Canvas<'a>>
}

pub enum PlotData<N> {
    Vector (Vec<[N; 2]>),
    Equation (Box<dyn Fn(N) -> Option<N>>),
}

impl<'a> Plot2D<'a> {
    pub fn new() -> Plot2D<'a> {
        Plot2D {
            cv: None
        }
    }
    
    pub fn check_cv(&self) -> Res<()> {
        if self.cv.is_none() {
            Err("No previously created Canvas found.")?
        } else {
            Ok(())
        }
    }
    
    pub fn save_canvas(&self, path: &str) -> Res<()> {
        self.check_cv()?;
        
        let cv = self.cv
            .as_ref()
            .unwrap();
        cv.save(path)?;
        
        return Ok(());
    }
}

impl<'a> Plot2D<'a> {
    pub fn make_canvas(
        &mut self,
        dim: Pos,
        bg_color: Pixel
    ) {
        let mut cv = Canvas::new(
            dim,
            bg_color
        );
        
        cv.set_layer(1)
            .unwrap();//TODO handle
        
        cv.add_node(Rect {
            offset: Pos (20, 20),
            dim: Pos (dim.0 -2*20, dim.1 -2*20),
            stroke: 2,
            color: [0,0,0,255],
            fill: false
        } ).unwrap();//TODO handle
        
        self.cv = Some(cv);
    }
    
    pub fn draw<N>(
        &mut self,
        data: PlotData<N>
    ) -> Res<()> 
    where  N : From<f64> + Into<f64> + PartialOrd<f64> + 'a
    {
        self.check_cv()?;
        
        let cv = self.cv.as_mut().unwrap();
        
        match data {
            PlotData::Vector(v) => {
                //TODO: Plot points
            },
            PlotData::Equation(eqn) => {
                cv.add_node(Rect {
                    offset: Pos (40, 40),
                    dim: Pos (2, 420),
                    stroke: 2,
                    color: [10,0,50,255],
                    fill: false
                })?;
                
                cv.add_node(Rect {
                    offset: Pos (40, 460),
                    dim: Pos (420, 2),
                    stroke: 2,
                    color: [10,0,50,255],
                    fill: false
                })?;
                
                cv.add_node(EqnQuad1 {
                    eqn,
                    origin: Pos (40,460),
                    dim: Pos (400, 400),
                    scale: (1_f32, 5_f32),
                    stroke: 2,
                    color: [255,0,0,255]
                })?;
            }
        };
        
        // let Canvas {
        //     dim: (w, h),
        //     ..
        // } = cv;
        
        return Ok(());
    }
}

pub fn test() {
    let data = PlotData::Equation(Box::new(
        |x : f64| -> Option<f64> {
            Some(2_f64*x)
        }
    ));
    
    let mut s = Plot2D::new();
    
    s.make_canvas(
        Pos (500, 500),
        [255; CH_LEN]
    );
    
    if let Err(why) = s.draw(data) {
        eprintln!("Error while drawing plot: {:?}", why);
    }
    
    if let Err(why) = s.save_canvas("tmp/plot.png") {
        eprintln!("Error while saving plot: {:?}", why);
    }
}