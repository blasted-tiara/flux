use crate::*;

const SHAKE_COUNT: u32 = 15;
const SHAKE_INTENSITY: i32 = 2;

#[turbo::serialize]
pub struct Hud {
    total_flux: f32,
    required_flux: f32,
    shake_count: u32,
    shake_x_offset: i32,
    shake_y_offset: i32,
}

impl Hud {
    pub fn new() -> Self {
        Hud {
            required_flux: 0.,
            total_flux: 0.,
            shake_count: 0,
            shake_x_offset: 0,
            shake_y_offset: 0,
        }
    }
    
    pub fn update(&mut self, total_flux: f32, required_flux: f32) {
        if (self.total_flux / FLUX_PER_UNIT) as i32 != (total_flux / FLUX_PER_UNIT) as i32 {
            self.shake_count = SHAKE_COUNT;
        } 
        self.total_flux = total_flux;
        self.required_flux = required_flux;

        if self.shake_count > 0 {
            self.shake_x_offset = random::i32() % SHAKE_INTENSITY;
            self.shake_y_offset = random::i32() % SHAKE_INTENSITY;
            self.shake_count -= 1;
        } else {
            self.shake_x_offset = 0;
            self.shake_y_offset = 0;
        }
    }
    
    pub fn draw(&self) {
        sprite!(
            "energybar",
            x = 0, 
            y = 0,
            fixed = true,
        );

        let mut flux_residue = self.total_flux % FLUX_PER_UNIT;
        if flux_residue <= 0. {
            flux_residue = FLUX_PER_UNIT - flux_residue.abs();
        }

        rect!(
            w = (231. * flux_residue / FLUX_PER_UNIT) as u32,
            h = 5,
            x = 144,
            y = 15,
            color = 0x0090ffff,
            border_radius = 2,
            fixed = true,
        );

        let mut units_of_flux = String::new();
        units_of_flux.push_str(&((self.total_flux / FLUX_PER_UNIT) as i32).to_string());

        text!(
            &units_of_flux,
            x = 31 - (units_of_flux.len() as i32) * 6 + self.shake_x_offset,
            y = 19 + self.shake_y_offset,
            color = 0xdddddddd,
            font = "VCRFont",
            fixed = true,
        );
        
        let mut required_flux_text = String::new();
        required_flux_text.push_str(&(self.required_flux as i32 / FLUX_PER_UNIT as i32).to_string());

        text!(
            &required_flux_text,
            x = 414 - (required_flux_text.len() as i32) * 5,
            y = 14,
            color = 0x2998c1ff,
            font = "large",
            fixed = true,
        );
    }
}