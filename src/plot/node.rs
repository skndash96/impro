use super::{
    Pos,
    Res,
    Pixel,
    //debug_img
};

pub struct Layer<'a> {
    pos: Pos,
    dim: Pos,
    bg: Pixel,
    lock: bool,
    nodes: Vec<Box<dyn Node + 'a>>
}

impl<'a> Layer<'a> {
    pub fn new(pos: Pos, dim: Pos, bg: Pixel, lock: bool) -> Self {
        Layer {
            pos,
            dim,
            bg,
            lock,
            nodes: vec![]
        }
    }
    
    pub fn lock(
        &mut self
    ) {
        self.lock = true;
    }
    
    pub fn unlock(
        &mut self
    ) {
        self.lock = false;
    }
    
    pub fn get_dim(&self) -> &Pos {
        &self.dim
    }
    pub fn get_bg(&self) -> &Pixel {
        &self.bg
    }
    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }
    
    pub fn check_lock(&self) -> Res<()> {
        if self.lock {
            Err("Failed to modify locked layer.")?
        } else {
            Ok(())
        }
    }
    
    pub fn push(
        &mut self,
        node: impl Node + 'a
    ) -> Res<()> {
        self.check_lock()?;
        
        self.nodes.push(
            Box::new(node)
        );
        
        return Ok(());
    }
    
    pub fn draw_u8(
        &self
    ) -> Res<Vec<u8>> {
        let Layer {
            dim,
            bg,
            ..
        } = *self;
        
        let mut raw : Vec<Pixel> = vec![bg; dim.0*dim.1];
        
        for node in &self.nodes {
            node.draw(
                &mut raw,
                dim
            )?;
        }
        
        return Ok(
            raw
            .flatten()
            .to_vec()
        );
    }
}

pub trait Node {
    fn draw(
        &self,
        raw: &mut Vec<Pixel>,
        cv_dim: Pos
    ) -> Res<()>;
}

pub struct Rect {
    pub offset: Pos,
    pub dim: Pos,
    pub stroke: u8,
    pub color: Pixel,
    pub fill: bool
}

pub struct Circ {
    pub offset: Pos,
    pub dim: Pos,
    pub stroke: u8,
    pub color: Pixel,
    pub fill: bool
}

pub struct EqnQuad1<T> {
    pub eqn: Box<dyn Fn(T) -> Option<T>>,
    pub origin: Pos,
    pub dim: Pos,
    pub scale: (f32, f32),
    pub stroke: u8,
    pub color: Pixel
}

impl<
    T: From<f64> + Into<f64> + PartialOrd<f64>
> Node for EqnQuad1<T> {
    fn draw(
        &self,
        raw: &mut Vec<Pixel>,
        Pos (cv_w, cv_h): Pos
    ) -> Res<()> {
        let f = &self.eqn;
        
        let EqnQuad1 {
            origin: Pos (off_x, max_y),
            dim: Pos (bw, bh),
            scale: (scale_x, scale_y),
            stroke,
            color,
            ..
        } = *self;
        
        //######## Tried to SMOOOTH
        // let threshold = 3_f32;
        // let n = stroke as i32 + threshold as i32;
        
        // let mix = |c: &Pixel, xd: i32, yd: i32| -> Pixel {
        //     let mut p = color; //rgba
            
        //     if xd < stroke as i32 && yd < stroke as i32 {
        //         return p;
        //     }
            
        //     let xd = xd as f32/n as f32;
        //     let yd = yd as f32/n as f32;
        //     let d = (xd + yd)/2_f32;
        //     //more d less color
            
        //     for i in 0..p.len() {
        //         p[i] = (
        //             c[i] as f32*(1_f32-d)
        //             + color[i] as f32*d
        //         ) as u8;
        //     }
            
        //     return p;
        // };
        
        for x in 0..bw {
            let x_idx = off_x + x;
            if x_idx > cv_w-1 {
                continue;
            }
            
            if let Some(y) = f((x as f64).into()) {
                if y > max_y as f64 || y > bh as f64 {
                    continue;
                }
                
                let y_idx = max_y-y.into() as usize;
                
                raw[y_idx*cv_w + x_idx] = color;
                
                // let start =
                //     (y_idx*cv_w) as i32
                //     + x_idx as i32
                //     - n*cv_w as i32
                //     - n as i32;
                
                // for yi in 0..n {
                //     let yidx = yi*cv_w as i32;
                    
                //     for xi in 0..n {
                //         let idx = (start+yidx+xi) as usize;
                        
                //         raw[idx] = mix(
                //             &raw[idx],
                //             xi, yi
                //         );
                //     }
                // }
            }
        }
        
        Ok(())
    }
}

impl Node for Circ {
    fn draw(
        &self,
        raw: &mut Vec<Pixel>,
        cv_dim: Pos
    ) -> Res<()> {
        let Circ {
            dim: box_dim,
            offset: off,
            stroke,
            color,
            fill
        } = *self;
        
        let (bw, bh) = box_dim.to_i32();
        let (off_x, off_y) = off.to_i32();
        let (cv_w, cv_h) = cv_dim.to_i32();
        
        let (org_x, org_y) = (
            off_x + bw/2,
            off_y + bh/2
        );
        
        let scale = 100*stroke as i32;
        
        let rad = bw/2 - bw/100;
        
        let check = |x: i32, y: i32, cur: &Pixel| -> Option<Pixel> {
            let n = (x-org_x).pow(2) + (y-org_y).pow(2) - rad.pow(2) as i32;
            
            let d = (n.abs() - scale) as f32 / 100_f32;
            //if d is less close to 1 more color
            
            if d < 0_f32
                || (fill && n < 0) {
                Some(color)
            } else if d < 1_f32 {
                let mut pix = color.clone();
                
                for i in 0..pix.len() {
                    let f = d*cur[i] as f32 + (1_f32-d)*pix[i] as f32;
                    pix[i] = f as u8;
                }
                // pix[3] = ((1_f32-d)*pix[3] as f32) as u8; //Rgba
                
                Some(pix)
            } else {
                None
            }
        };
        
        for ri in 0..cv_h {
            if ri < off_y
                || ri > off_y + bh -1 {
                continue;
            }
            
            for ci in 0..cv_w {
                if ci < off_x
                    || ci > off_x + bw -1 {
                    continue;
                }
                
                let idx = (ri*cv_w + ci) as usize;
                
                if let Some(pix) = check(ci, ri, &raw[idx]) {
                    raw[idx] = pix;
                }
            }
        }
        
        return Ok(());
    }
}

impl Node for Rect {
    //Box width and height includes the rectangle stroke width
    //Like CSS's box-sizing: border box;
    fn draw(
        &self,
        raw: &mut Vec<Pixel>,
        Pos (cv_w, cv_h): Pos
    ) -> Res<()> {
        let Rect {
            dim: Pos (bw, bh),
            offset: Pos (off_x, off_y),
            stroke,
            color,
            fill
        } = *self;
        
        let stroke = stroke as usize;
        
        let bor_slice = vec![color; stroke];
        
        for ri in 0..cv_h {
            if ri < off_y
                || ri > off_y +bh -1 {
                continue;
            }
            
            let back = ri*cv_w + off_x;
            
            if fill
                || ri < off_y+stroke
                || ri > off_y+bh-stroke-1 {
                raw.splice(back..(back +bw), vec![color; bw]);
            } else {
                raw.splice(back..(back +stroke), bor_slice.clone());
                raw.splice((back + bw-stroke)..(back + bw), bor_slice.clone());
            }
        }
        
        return Ok(());
    }
}
