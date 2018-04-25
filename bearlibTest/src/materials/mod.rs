use Materials;
use num_traits::pow;


#[derive(Clone)]
pub struct Material {
    Name: String,
    SPEC_HEAT: Option<i32>,
    MELTING_POINT: Option<i32>,
	BOILING_POINT: Option<i32>,
    HEATDAM_POINT: Option<i32>,
    IGNITE_POINT: Option<i32>,
	COLDDAM_POINT: Option<i32>,
	SOLID_DENSITY: Option<i32>,
	LIQUID_DENSITY: Option<i32>,
	MOLAR_MASS: Option<i32>,
	IMPACT_YIELD: Option<i32>,
	IMPACT_FRACTURE: Option<i32>,
	IMPACT_STRAIN_AT_YIELD: Option<i32>,
	COMPRESSIVE_YIELD: Option<i32>,
	COMPRESSIVE_FRACTURE: Option<i32>,
	COMPRESSIVE_STRAIN_AT_YIELD: Option<i32>,
	TENSILE_YIELD: Option<i32>,
	TENSILE_FRACTURE: Option<i32>,
	TENSILE_STRAIN_AT_YIELD: Option<i32>,
	TORSION_YIELD: Option<i32>,
	TORSION_FRACTURE: Option<i32>,
	TORSION_STRAIN_AT_YIELD: Option<i32>,
	SHEAR_YIELD: Option<i32>,
	SHEAR_FRACTURE: Option<i32>,
	SHEAR_STRAIN_AT_YIELD: Option<i32>,
	BENDING_YIELD: Option<i32>,
	BENDING_FRACTURE: Option<i32>,
	BENDING_STRAIN_AT_YIELD: Option<i32>,
	MAX_EDGE: Option<i32>,
}
    // density = weight * volume


/*enum Materials {
    Steel,
}*/


pub fn fetch_material(x: Materials, cmThick: f32, cubicFlag: bool) -> Material {
    
    let dens: f32;
    
    if cubicFlag {
        dens = 8.0 * cmThick;
    } else {
        dens = 8.0 * cmThick;//.pow(3);
    }

    let STEEL: Material = Material {
        Name: "Steel".to_string(),
        SPEC_HEAT: Some(500),
        MELTING_POINT: Some(12718),
	    BOILING_POINT: Some(14968),
	    SOLID_DENSITY: Some(7850),
	    LIQUID_DENSITY: Some(6980),
        COLDDAM_POINT: None,
        HEATDAM_POINT: None,
        IGNITE_POINT: None,
	    MOLAR_MASS: Some(55845),
	    IMPACT_YIELD: Some(1505000),
	    IMPACT_FRACTURE: Some(2520000),
	    IMPACT_STRAIN_AT_YIELD: Some(940),
	    COMPRESSIVE_YIELD: Some(1505000),
	    COMPRESSIVE_FRACTURE: Some(2520000),
	    COMPRESSIVE_STRAIN_AT_YIELD: Some(940),
	    TENSILE_YIELD: Some(430000),
	    TENSILE_FRACTURE: Some(720000),
	    TENSILE_STRAIN_AT_YIELD: Some(225),
	    TORSION_YIELD: Some(430000),
	    TORSION_FRACTURE: Some(720000),
	    TORSION_STRAIN_AT_YIELD: Some(215),
	    SHEAR_YIELD: Some(430000),
	    SHEAR_FRACTURE: Some(720000),
	    SHEAR_STRAIN_AT_YIELD: Some(215),
	    BENDING_YIELD: Some(430000),
	    BENDING_FRACTURE:Some(720000),
	    BENDING_STRAIN_AT_YIELD: Some(215),
	    MAX_EDGE: Some(10000),
    };

/*    [URIST] = [FAHRENHEIT] + 9968
                   [CELSIUS] * 9/5 + 10000 
                   [KELVIN] * 9/5 + 9508.33 
                   [RANKINE] + 9508.33 

                   (urist - 10000) / (9/5)
*/

    let SKIN: Material = Material {
        Name: "Skin".to_string(),
        SPEC_HEAT: Some(4181),
        IGNITE_POINT: Some(10508),
	    MELTING_POINT: None,
	    BOILING_POINT: None,
	    HEATDAM_POINT: Some(10250),
	    COLDDAM_POINT: Some(9900),
	    SOLID_DENSITY: Some(1000),
	    LIQUID_DENSITY: None,
	    MOLAR_MASS: None,
	    IMPACT_YIELD: Some(10000),
	    IMPACT_FRACTURE: Some(10000),
	    IMPACT_STRAIN_AT_YIELD: Some(500000),
	    COMPRESSIVE_YIELD: Some(10000),
	    COMPRESSIVE_FRACTURE: Some(10000),
	    COMPRESSIVE_STRAIN_AT_YIELD: Some(50000),
	    TENSILE_YIELD: Some(10000),
	    TENSILE_FRACTURE: Some(10000),
	    TENSILE_STRAIN_AT_YIELD: Some(50000),
	    TORSION_YIELD: Some(10000),
	    TORSION_FRACTURE: Some(10000),
	    TORSION_STRAIN_AT_YIELD: Some(50000),
	    SHEAR_YIELD: Some(20000),
	    SHEAR_FRACTURE: Some(20000),
	    SHEAR_STRAIN_AT_YIELD: Some(50000),
	    BENDING_YIELD: Some(10000),
	    BENDING_FRACTURE: Some(10000),
	    BENDING_STRAIN_AT_YIELD: Some(50000),
	    MAX_EDGE: Some(0),
    };

    let BONE: Material = Material {
        Name: "Bone".to_string(),
        SPEC_HEAT: Some(1000),
	    IGNITE_POINT: Some(10508),
	    MELTING_POINT: None,
	    BOILING_POINT: None,
	    HEATDAM_POINT: Some(10250),
	    COLDDAM_POINT: Some(9900),
	    SOLID_DENSITY: Some(500),
	    LIQUID_DENSITY: None,
	    MOLAR_MASS: None,
	    IMPACT_YIELD: Some(200000),
	    IMPACT_FRACTURE: Some(200000),
	    IMPACT_STRAIN_AT_YIELD: Some(100),
	    COMPRESSIVE_YIELD: Some(200000),
	    COMPRESSIVE_FRACTURE: Some(200000),
	    COMPRESSIVE_STRAIN_AT_YIELD: Some(100),
	    TENSILE_YIELD: Some(115000),
	    TENSILE_FRACTURE: Some(130000),
	    TENSILE_STRAIN_AT_YIELD: Some(100),
	    TORSION_YIELD: Some(115000),
	    TORSION_FRACTURE: Some(130000),
	    TORSION_STRAIN_AT_YIELD: Some(100),
	    SHEAR_YIELD: Some(115000),
	    SHEAR_FRACTURE: Some(130000),
	    SHEAR_STRAIN_AT_YIELD: Some(100),
	    BENDING_YIELD: Some(115000),
	    BENDING_FRACTURE: Some(130000),
	    BENDING_STRAIN_AT_YIELD: Some(100),
	    MAX_EDGE: Some(1000),
    };

    let MUSCLE: Material = Material {
        Name: "Muscle".to_string(),
        SPEC_HEAT: Some(4181),
	    IGNITE_POINT: Some(10508),
	    MELTING_POINT: None,
	    BOILING_POINT: None,
	    HEATDAM_POINT: Some(10250),
	    COLDDAM_POINT: Some(9900),
	    SOLID_DENSITY: Some(1060),
	    LIQUID_DENSITY: None,
	    MOLAR_MASS: None,
	    IMPACT_YIELD: Some(10000),
	    IMPACT_FRACTURE: Some(10000),
	    IMPACT_STRAIN_AT_YIELD: Some(50000),
	    COMPRESSIVE_YIELD: Some(10000),
	    COMPRESSIVE_FRACTURE: Some(10000),
	    COMPRESSIVE_STRAIN_AT_YIELD: Some(50000),
	    TENSILE_YIELD: Some(10000),
	    TENSILE_FRACTURE: Some(10000),
	    TENSILE_STRAIN_AT_YIELD: Some(50000),
	    TORSION_YIELD: Some(10000),
	    TORSION_FRACTURE: Some(10000),
	    TORSION_STRAIN_AT_YIELD: Some(50000),
	    SHEAR_YIELD: Some(20000),
	    SHEAR_FRACTURE: Some(20000),
	    SHEAR_STRAIN_AT_YIELD: Some(50000),
	    BENDING_YIELD: Some(10000),
	    BENDING_FRACTURE: Some(10000),
	    BENDING_STRAIN_AT_YIELD: Some(50000),
	    MAX_EDGE: Some(0),
    };

    let TISSUE: Material = Material {
        Name: "Tissue".to_string(),
	    SPEC_HEAT: Some(4181),
	    IGNITE_POINT: Some(10508),
	    MELTING_POINT: None,
	    BOILING_POINT: None,
	    HEATDAM_POINT: Some(10250),
	    COLDDAM_POINT: Some(9900),
	    SOLID_DENSITY: Some(500),
	    LIQUID_DENSITY: None,
	    MOLAR_MASS: None,
	    IMPACT_YIELD: Some(10000),
	    IMPACT_FRACTURE: Some(10000),
	    IMPACT_STRAIN_AT_YIELD: Some(50000),
	    COMPRESSIVE_YIELD: Some(10000),
	    COMPRESSIVE_FRACTURE: Some(10000),
	    COMPRESSIVE_STRAIN_AT_YIELD: Some(50000),
	    TENSILE_YIELD: Some(10000),
	    TENSILE_FRACTURE: Some(10000),
	    TENSILE_STRAIN_AT_YIELD: Some(50000),
	    TORSION_YIELD: Some(10000),
	    TORSION_FRACTURE: Some(10000),
	    TORSION_STRAIN_AT_YIELD: Some(50000),
	    SHEAR_YIELD: Some(20000),
	    SHEAR_FRACTURE: Some(20000),
	    SHEAR_STRAIN_AT_YIELD: Some(50000),
	    BENDING_YIELD: Some(10000),
	    BENDING_FRACTURE: Some(10000),
	    BENDING_STRAIN_AT_YIELD: Some(50000),
	    MAX_EDGE: Some(0),
    };

	let FAT: Material = Material {
		Name: "Tissue".to_string(),
		SPEC_HEAT: Some(4181),
		IGNITE_POINT: Some(10338),
		MELTING_POINT: Some(10078),
		BOILING_POINT: None,
		HEATDAM_POINT: Some(10250),
		COLDDAM_POINT: Some(9900),
		SOLID_DENSITY: Some(900),
		LIQUID_DENSITY: Some(800),
		MOLAR_MASS: None,
		IMPACT_YIELD: Some(10000),
		IMPACT_FRACTURE: Some(10000),
		IMPACT_STRAIN_AT_YIELD: Some(50000),
		COMPRESSIVE_YIELD: Some(10000),
		COMPRESSIVE_FRACTURE: Some(10000),
		COMPRESSIVE_STRAIN_AT_YIELD: Some(50000),
		TENSILE_YIELD: Some(10000),
		TENSILE_FRACTURE: Some(10000),
		TENSILE_STRAIN_AT_YIELD: Some(50000),
		TORSION_YIELD: Some(10000),
		TORSION_FRACTURE: Some(10000),
		TORSION_STRAIN_AT_YIELD: Some(50000),
		SHEAR_YIELD: Some(10000),
		SHEAR_FRACTURE: Some(10000),
		SHEAR_STRAIN_AT_YIELD: Some(50000),
		BENDING_YIELD: Some(10000),
		BENDING_FRACTURE: Some(10000),
		BENDING_STRAIN_AT_YIELD: Some(50000),
		MAX_EDGE: Some(0),
	};

    match x {
        Materials::Steel => STEEL,
        Materials::Bone => BONE,
        Materials::Skin => SKIN,
        Materials::Muscle => MUSCLE,
        Materials::Tissue => TISSUE,
		Materials::Fat => FAT,
		_ => FAT
    }

}
