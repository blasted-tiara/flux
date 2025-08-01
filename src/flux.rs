use crate::*;

#[turbo::serialize]
pub struct Flux {
    strength: f32,
    pub solid: Solid,
}

impl Flux {
    pub fn new(strength: f32, position: Vector2, width: f32, height: f32) -> Self {
        Self {
            strength,
            solid: Solid {
                position,
                width,
                height,
            }
        }
    }
    
    pub fn draw(&self) {
        let x = (self.solid.position.x - self.solid.width / 2.) as i32;
        let y = (self.solid.position.y - self.solid.height / 2.) as i32;

        if self.strength > 0. {
            sprite!("flux_source", x = x, y = y, scale = SPRITE_SCALE);
        } else if self.strength < 0. {
            sprite!("flux_sink", x = x, y = y, scale = SPRITE_SCALE);
        } else {
            sprite!("dirt", x = x, y = y, scale = SPRITE_SCALE);
        }
    }
}

pub fn calculate_line_flux(start: &Vector2, end: &Vector2, segment_count: u32, flux_cores: &Vec<Flux>) -> Vector2 {
    let segments = line_to_segments(start, end, segment_count);
    let mut net_flux = Vector2::zero();
    
    for i in 0..segment_count {
        let start_option = segments.get(i as usize);
        let end_option = segments.get((i + 1) as usize);
        match (start_option, end_option) {
            (Some(start), Some(end)) => {
                net_flux += calculate_line_segment_flux(start, end, flux_cores);
            },
            (_, _) => {
                log!("Calculate line flux: mismatched line segment count!");
            }
        }
    }
    
    net_flux
}

pub fn net_flux_field_at_point(point: &Vector2, flux_cores: &Vec<Flux>) -> Vector2 {
    let mut total_flux = Vector2::zero();
    for flux_core in flux_cores  {
        total_flux += flux_field_at_point(point, &flux_core)     
    }
    
    total_flux
}

fn flux_field_at_point(point: &Vector2, flux_core: &Flux) -> Vector2 {
    let r = point - &flux_core.solid.position;
    r * (flux_core.strength / (2. * PI * r.length_squared()))
}

pub fn line_to_segments(start: &Vector2, end: &Vector2, segment_count: u32) -> Vec<Vector2> {
    let mut segments = vec![start.clone()];
    let vector = end - start;
    let increment = vector * (1. / segment_count as f32);
    
    for i in 0..segment_count {
        let segment_point = start + (increment * (i + 1) as f32);
        segments.push(segment_point); 
    }
    
    segments
}

pub fn calculate_line_segment_flux(start: &Vector2, end: &Vector2, flux_cores: &Vec<Flux>) -> Vector2 {
    let mid_point = start + ((end - start) * 0.5);
    let delta = (end - start).length();
    
    net_flux_field_at_point(&mid_point, flux_cores) * delta
}

pub fn get_flux_line(rotation: f32, bounding_box: &BoundingBox) -> (Vector2, Vector2) {
    let center = Vector2::new(bounding_box.left + (bounding_box.right - bounding_box.left) / 2., bounding_box.top + (bounding_box.bottom - bounding_box.top) / 2.);
    let start = Vector2::new(bounding_box.right, bounding_box.top + (bounding_box.bottom - bounding_box.top) / 2.);
    let end = Vector2::new(bounding_box.left, bounding_box.top + (bounding_box.bottom - bounding_box.top) / 2.);
    
    let rotated_start = start.rotate_point(center, rotation);
    let rotated_end = end.rotate_point(center, rotation);

    (rotated_start, rotated_end)    
}

pub fn show_total_flux(total_flux: f32, screen_center: &Vector2) {
    let mut a = "Total flux: ".to_owned();
    a.push_str(&(total_flux.to_string()));
    text!(
        &a,
        x = screen_center.x - 250.,
        y = screen_center.y - 125., 
        font = "large",
        color = 0x556677ff,
    );
    
    rect!(
        w = 20,
        h = 100,
        x = screen_center.x - 250.,
        y = screen_center.y - 100.,
        color = 0xc85dd9ff,
    );

    rect!(
        w = 16,
        h = 96,
        x = screen_center.x - 248.,
        y = screen_center.y - 98.,
        color = 0x630f75ff,
    );
    
    let bar_height = 96. * total_flux / 1200.;
    if total_flux > 0. {
        rect!(
            w = 16,
            h = bar_height,
            x = screen_center.x - 248.,
            y = screen_center.y - 98. + 96. / 2. - bar_height,
            color = 0xff0000ff,
        )
    } else {
        rect!(
            w = 16,
            h = -bar_height,
            x = screen_center.x - 248.,
            y = screen_center.y - 98. + 96. / 2.,
            color = 0x0000ffff,
        )
    }
}