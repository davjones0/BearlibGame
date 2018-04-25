use num_traits::pow;
use num_traits::cast::ToPrimitive;
use std::f64::consts;

// 50J defeat padding 16 layers linen
// 50J to serious injury
// 100J to defeat padding with blade
// modifiers to multiply by
// Munition quality iron: 0.5
// Low-carbon steel: 0.75
// Medium-carbon steel (Milanese): 1.1
// Hardened steel: 1.5 

// cavital strength = 3x yield strength

// armor thickness and diameter in m
// mass of projectile in kg
// angle of impact in degrees

/* PF (tonne) = Perimeter (mm) x Thickness (mm) x Shear Strength (N/mm2) / 9806.65 (N/tonne)
*/
fn kinetic_energy(mass: f32, vel: i32) -> f32 {
    // kinetic energy in Joules
    // mass in kilograms
    // velocity in m/s
    let output = (1.0/2.0) * mass * pow((vel as f32), 2);
    output
}

fn work_performed(avgForce: f32, distPen: f32) -> f32 {
    
    let output = avgForce * distPen;
    output
}

fn f_coefficient(thickness: f32, diameter: f32, impactAngle: f32) -> f32 {

    let output = 1.8288 * (thickness / diameter - 0.45) * (pow(impactAngle, 2) + 2000.0) + 12192.0;
    output
}

fn energy_req_pentration(impactToughness: i32, thicknessCM: f32, areaBullet: f32) -> f32 {
    // energy required to penetrate in Joules
    // impact toghness in Joules/cm
    // thickness in cm
    // area of bullet/cross section in cm
    let output = (impactToughness as f32) * thicknessCM * areaBullet;
    output
}


// 9mm = pi * (.355/2)^2
// 0.2514087009916 cm
//velocity 380.0 m/s
//mass 7.5 g/ .0075 kg

#[test]
fn physics_tests() {

    //assert_eq!(kinetic_energy(), );
    //9mm
    assert_eq!(kinetic_energy(0.0075, 380), 541.5);
    
    //9mm
    assert_eq!(energy_req_pentration(1800, 0.635, 0.25), 6.75);

    //assert_eq!(energy_req_pentration(), );

}
