use Materials;

#[derive(Clone)]
pub struct Material {
    Name: String,
    Toughness: i32,
    Boiling_Point: i32,
    Melting_Point: i32,
    Ignition_Point: i32,
    Density: i32,
    Freezing_Point: i32,
}
    // density = weight * volume


/*enum Materials {
    Steel,
}*/



pub fn fetch_material(x: Materials) -> Material {
    let STEEL: Material = Material {
        Name: "Steel".to_string(),
        Toughness: 27,
        Boiling_Point: 2750,
        Melting_Point: 1300,
        Ignition_Point: 0,
        Density: 0,
        Freezing_Point: 1290,
    };


    match x {
        Materials::Steel => STEEL
    }

}