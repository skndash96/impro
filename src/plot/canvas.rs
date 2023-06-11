use std::fs::File;

use image::ColorType;
use image::ImageEncoder;
use image::codecs::png::PngEncoder;

use crate::ops::doub::overlay;
use crate::ops::{
    PNG_FILTER,
    PNG_COMPRESSION
};

use super::{
    Pos,
    Pixel,
    Res
};

use super::node::{
    Node,
    Layer
};

pub struct Canvas<'a> {
    dim: Pos,
    layers: Vec<Layer<'a>>,
    //layer 0 is background
    cur_layer: u8
}

impl<'a> Canvas<'a> {
    pub fn new(
        dim: Pos,
        color: Pixel
    ) -> Self {
        Canvas {
            dim,
            layers: vec![
                Layer::new(
                    Pos (0,0),
                    dim,
                    color,
                    true
                ),
                Layer::new(
                    Pos (0,0),
                    dim,
                    [0,0,0,0],
                    false
                )
            ],
            cur_layer: 1
        }
    }
    
    pub fn set_layer(
        &mut self,
        n: u8
    ) -> Res<()> {
        if n as usize >= self.layers.len() {
            Err("Given layer index is out of bounds.")?
        }
        
        self.cur_layer = n;
        
        Ok(())
    }
    
    pub fn add_node(
        &mut self,
        node: impl Node + 'a
    ) -> Res<()> {
        self.layers[self.cur_layer as usize]
            .push(node)?;
        
        Ok(())
    }
    
    pub fn save(
        &self,
        path: &str
    ) -> Res<()> {
        let layers = &self.layers;
        
        let base_dim = layers[0].get_dim();
        let base_bg = layers[0].get_bg();
        
        let mut raw = layers[0].draw_u8()?;
        
        for idx in 1..layers.len() {
            let l = &layers[idx];
            let raw2 = l.draw_u8()?;
            
            overlay(
                &mut raw,
                base_dim.to_u32(),
                &raw2,
                l.get_dim().to_u32(),
                l.get_pos().to_i32()
            )?;
        }
        
        let write_to = File::create(path)?;
        
        let png = PngEncoder::new_with_quality(
            write_to,
            PNG_COMPRESSION,
            PNG_FILTER
        );
        
        png.write_image(
            &raw[..],
            base_dim.0 as u32,
            base_dim.1 as u32,
            ColorType::Rgba8
        )?;
        
        return Ok(());
    }
}
