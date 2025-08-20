use crate::*;

pub fn draw_hud(total_flux: f32, required_flux: f32) {
    sprite!(
        "energybar",
        x = 0, 
        y = 0,
        fixed = true,
    );

    let mut flux_residue = total_flux % FLUX_PER_UNIT;
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
    units_of_flux.push_str(&((total_flux / FLUX_PER_UNIT) as i32).to_string());

    text!(
        &units_of_flux,
        x = 31 - (units_of_flux.len() as i32) * 6,
        y = 19,
        color = 0xffffffff,
        font = "VCRFont",
        fixed = true,
    );
    
    let mut required_flux_text = String::new();
    required_flux_text.push_str(&(required_flux as i32 / FLUX_PER_UNIT as i32).to_string());

    text!(
        &required_flux_text,
        x = 420 - (required_flux_text.len() as i32) * 5,
        y = 15,
        color = 0x2998c1ff,
        font = "medium",
        fixed = true,
    );
    
}