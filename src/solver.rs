const TIME_STEP: f64 = 1e-2;


#[derive(Debug, Copy, Clone)]
pub struct Position(pub f64, pub f64, pub f64);
#[derive(Debug, Copy, Clone)]
struct Velocity(f64, f64, f64);
#[derive(Debug)]
struct ElectricField(f64, f64, f64);
#[derive(Debug)]
struct MagneticField(f64, f64, f64);

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pos: Position,
    vel: Velocity,
    m: f64,
    q: f64,
}

fn build_particle(pos: Position, vel: Velocity, m: f64, q: f64) -> Particle {
    Particle {
        pos,
        vel,
        m,
        q,
    }
}

fn push_particle(part: &mut Particle) {
    part.pos.0 = part.pos.0 + TIME_STEP * part.vel.0;
    part.pos.1 = part.pos.1 + TIME_STEP * part.vel.1;
    part.pos.2 = part.pos.2 + TIME_STEP * part.vel.2;
}

fn kick_particle(part: &mut Particle, e_field: &ElectricField, b_field: &MagneticField, dt: f64) {
    part.vel.0 = part.vel.0 + dt * part.q/part.m * ( e_field.0 + part.vel.1 * b_field.2 - part.vel.2 * b_field.1 );
    part.vel.1 = part.vel.1 + dt * part.q/part.m * ( e_field.1 + part.vel.2 * b_field.0 - part.vel.0 * b_field.2 );
    part.vel.2 = part.vel.2 + dt * part.q/part.m * ( e_field.2 + part.vel.0 * b_field.1 - part.vel.1 * b_field.0 );
}

pub fn solve() -> [Position; 1000] {
    let e_field = &ElectricField(0., 0., 0.);
    let b_field = &MagneticField(0., 0., 1.);
    let mut part = build_particle( Position(1., 0., 0.), Velocity(0., 1., 0.), 1., -1. );
    let mut res = [part.pos.clone(); 1000];

    kick_particle(&mut part, e_field, b_field, -0.5 * TIME_STEP);

    for i in 0..999 {
        kick_particle(&mut part, e_field, b_field, TIME_STEP);
        push_particle(&mut part);

        if i % 10 == 0 {
            res[i] = part.pos.clone();
        }
    }

    res
}
