use crate::*;

const MAX_PARTICLES: u32 = 1000;
const FLUX_FIELD_CONSTANT: f32 = 0.15;
const MAX_PARTICLE_SPEED: f32 = 2.5;
const MAX_PARTICLE_TAIL_COUNT: u32 = 9;

#[turbo::serialize]
pub struct ParticleManager {
    particle_pool: Vec<Particle>,
}

impl ParticleManager {
    pub fn new() -> Self {
        Self {
            particle_pool: vec![],
        }
    }
    
    pub fn update(&mut self, flux_cores: &Vec<FluxCore>) {
        let mut dead_particle_indices: Vec<usize> = Vec::new();
        for (idx, particle) in &mut self.particle_pool.iter_mut().enumerate() {
            particle.update(flux_cores);
            if !particle.is_alive {
                dead_particle_indices.push(idx);
            }
        }
        
        while let Some(idx) = dead_particle_indices.pop() {
            let last_element_idx = self.particle_pool.len() - 1;
            self.particle_pool.swap(idx,    last_element_idx);
            self.particle_pool.pop();
        }
    }
    
    pub fn draw(&self) {
        for particle in &self.particle_pool {
            if particle.is_alive {
                particle.draw();
            } else {
                // This is fine because all the active particles will be on the left side
                break;
            }
        }
    }
    
    pub fn generate_box_of_particles(&mut self, n: u32, bounding_box: &BoundingBox) {
        if self.particle_pool.len() as u32 + n > MAX_PARTICLES {
            return;
        }
        for _i in 0..n {
            let x = random::between(bounding_box.left, bounding_box.right);
            let y = random::between(bounding_box.top, bounding_box.bottom);
            self.particle_pool.push(Particle::new(x, y, (3.0 * 60.) as u32));
        }
    }
}

#[turbo::serialize]
pub struct Particle {
    is_alive: bool,
    jitter: f32,
    diameter: u32,
    color: u32,
    positions: VecDeque<Vector2>,
    lifetime: u32,
    update_interval: u32,
    interval_count: u32,
}

impl Particle {
    pub fn new(x: f32, y: f32, lifetime: u32) -> Self {
        Self {
            is_alive: true,
            jitter: random::between(0.1, 0.8),
            diameter: 1 + random::u32() % 3,
            color: random_color(1.),
            positions: {
                let mut deque: VecDeque<Vector2> = VecDeque::new();
                deque.push_back(Vector2::new(x, y));
                deque
            },
            update_interval: 2,
            interval_count: 0,
            lifetime,
        }
    }
    
    fn update(&mut self, flux_cores: &Vec<FluxCore>) {
        if !self.is_alive {
            // TODO: Particle should disappear gracefuly
            return;
        }

        self.lifetime -= 1;
        if self.lifetime == 0 {
            self.is_alive = false;
            return;
        }
        
        if self.interval_count < self.update_interval {
            self.interval_count += 1;
            return;
        } else {
            self.interval_count = 0;
        }

        match self.positions.back() {
            Some(particle) => {
                let prev_position = particle.clone();
                let new_position = self.random_walk(prev_position);
                let net_flux_field = net_flux_field_at_point(particle, &flux_cores);
                self.positions.push_back(new_position + (net_flux_field * FLUX_FIELD_CONSTANT).clamp_length(MAX_PARTICLE_SPEED));
            },
            None => {},
        }
        if self.positions.len() >= MAX_PARTICLE_TAIL_COUNT as usize {
            self.positions.pop_front();
        }
    }

    fn random_walk(&self, position: Vector2) -> Vector2 {
        position + Vector2::random() * self.jitter
    }
    
    fn draw(&self) {
        // NOTE: This can be optimized to 
        let mut i = 1;
        let mut alpha = 0.6;
        for position in &self.positions {
            circ!{
                d = lerp(0., self.diameter as f32, i as f32 / MAX_PARTICLE_TAIL_COUNT as f32).ceil(),
                x = position.x,
                y = position.y,
                color = change_alpha(self.color, alpha),
            }
            i += 1;
            alpha += 0.1;
        } 
    }
}

pub fn random_color(alpha: f32) -> u32 {
    let r = random::between(0.2, 0.4) + if (tick() % 500) > 250 { 0.0 } else { 0.4 };
    let g = random::between(0.6, 1.0);
    let b = random::between(0.7, 1.0);
    color_rgb(r, g, b, alpha)
}

pub fn color_rgb(red: f32, green: f32, blue: f32, alpha: f32) -> u32 {
    ((red.clamp(0., 1.) * 255.) as u32) << 24 |
    ((green.clamp(0., 1.) * 255.) as u32) << 16 |
    ((blue.clamp(0., 1.) * 255.) as u32) << 8 |
    ((alpha.clamp(0., 1.) * 255.) as u32)
}

pub fn change_color(color: u32, red: f32, green: f32, blue: f32, alpha: f32) -> u32 {
    (((color & 0xff) as f32 + red * 255.) as u32).clamp(0, 255) << 24 |
    (((color & 0xff) as f32 + green * 255.) as u32).clamp(0, 255) << 16 |
    (((color & 0xff) as f32 + blue * 255.) as u32).clamp(0, 255) << 8 |
    (((color & 0xff) as f32 + alpha * 255.) as u32).clamp(0, 255)
}

pub fn change_alpha(color: u32, alpha: f32) -> u32 {
    if alpha >= 0. && alpha <= 1. {
        (color & 0xffffff00) | ((alpha * 256.) as u32 & 0x000000ff)
    } else {
        color
    }
}