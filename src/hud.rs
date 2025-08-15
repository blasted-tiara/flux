use crate::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const FLUX_PER_UNIT: i32 = 200;

pub fn draw_hud(camera_center_x: f32, camera_center_y: f32, total_flux: f32) {
    sprite!(
        "energybargb",
        x = 0, 
        y = 0,
        fixed = true,
    );

    let mut flux_residue = ((total_flux % FLUX_PER_UNIT as f32) / (FLUX_PER_UNIT / 8) as f32).ceil() as i32;
    if flux_residue <= 0 {
        flux_residue = 8 - flux_residue.abs();
    }
    let mut eigths_of_flux = String::from("eb_arrow_");
    eigths_of_flux.push_str(&flux_residue.to_string());
    sprite!(
        &eigths_of_flux,
        x = 0, 
        y = 0,
        fixed = true,
    );

    let mut hundreds_of_flux = String::new();
    hundreds_of_flux.push_str(&(total_flux as i32 / FLUX_PER_UNIT).to_string());

    text!(
        &hundreds_of_flux,
        x = 97 - (hundreds_of_flux.len() as i32) * 5,
        y = 15,
        color = 0x00305dff,
        font = "large",
        fixed = true,
    );
}