use Materials;

#[derive(Copy, Clone, Debug)]
pub struct damage_states {
    bruised: f32,
    burned: f32,
    frostbite: f32,
    cut: f32
}

impl damage_states {
    // add in function to add damage values here
    pub fn new() -> damage_states {
        damage_states {
            bruised: 0.0,
            burned: 0.0,
            frostbite: 0.0,
            cut: 0.0
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layer {
    Name: String,
    Material: Materials,
    Thickness: f32,
    Status: String,
    Damage: damage_states,
}

impl Layer {
    pub fn new(name: String, material: Materials, thick: f32, status: String, damage: damage_states) -> Layer {
        Layer {
            Name: name,
            Material: material,
            Thickness: thick,
            Status: status,
            Damage: damage
        }
    }

    pub fn get_Name(&self) -> &str {
        &self.Name
    } 

    pub fn get_Material(&self) -> Materials {
        self.Material
    } 

    pub fn get_Thickness(&self) -> f32 {
        self.Thickness
    } 

    pub fn get_Damage(&self) -> damage_states {
        self.Damage
    } 

}

/*
body_head
body_neck
body_upperbody
body_lowerbody
body_leftleg
body_rightleg
body_leftfoot
body_rightfoot
body_rightsholder
body_leftsholder
body_rightupperarm
body_leftupperarm
body_leftlowerarm
body_rightlowerarm
body_righthand
body_lefthand
body_tail
*/

pub fn generate_layer(name: String ,layer_type: Materials, thickness: f32) -> Layer {
    Layer::new(name, layer_type, thickness, " ".to_string(), damage_states::new())
}

pub fn human_head_layers() -> Vec<Layer> {
    let mut layerStash = Vec::new();
    let skin = generate_layer("Skin".to_string(), Materials::Skin, 2.0);
    let fat = generate_layer("Fat".to_string(), Materials::Skin, 2.0);
    let muscle = generate_layer("Muscle".to_string(),Materials::Muscle, 1.1);
    let bone = generate_layer("Skull".to_string(), Materials::Bone, 2.0);
    //let organ_placement = generate_layer("Organs".to_string(), Materials::None, 4.0);

    layerStash.push(skin);
    layerStash.push(fat);
    layerStash.push(muscle);
    layerStash.push(bone);
    //layerStash.push(organ_placement);

    layerStash

}

pub fn human_neck_layers() -> Vec<Layer> {
    let mut layerStash = Vec::new();
    
    let skin = generate_layer("Skin".to_string(), Materials::Skin, 2.0);
    let fat = generate_layer("Fat".to_string(), Materials::Fat, 2.0);
    let muscle = generate_layer("Muscle".to_string(), Materials::Muscle, 1.1);
    let bone = generate_layer("Skull".to_string(), Materials::Bone, 2.0);

    layerStash.push(skin);
    layerStash.push(fat);
    layerStash.push(muscle);
    layerStash.push(bone);

    layerStash
}

pub fn human_upperbody_layers() -> Vec<Layer> {
    let mut layerStash = Vec::new();
    
    let skin = generate_layer("Skin".to_string(), Materials::Skin, 2.0);
    let fat = generate_layer("Fat".to_string(), Materials::Fat, 2.0); 
    let muscle = generate_layer("Muscle".to_string(), Materials::Muscle, 1.1);
    let bone = generate_layer("Rib".to_string(), Materials::Bone, 2.0);
    let organ_placement = generate_layer("Organs".to_string(), Materials::None, 4.0);


    layerStash.push(skin);
    layerStash.push(fat);
    layerStash.push(muscle);
    layerStash.push(bone);

    layerStash
}

pub fn human_lowerbody_layers() -> Vec<Layer> {
    let mut layerStash = Vec::new();

    let skin = generate_layer("Skin".to_string(), Materials::Skin, 2.0);
    let fat = generate_layer("Fat".to_string(), Materials::Fat, 2.0);
    let muscle = generate_layer("Muscle".to_string(), Materials::Muscle, 1.1);
    //let organ_placement = generate_layer("Organs".to_string(), Materials::None, 4.0);
    let bone = generate_layer("Rib".to_string(), Materials::Bone, 2.0);

    layerStash.push(skin);
    layerStash.push(fat);
    layerStash.push(muscle);
    //layerStash.push(organ_placement);
    layerStash.push(bone);

    layerStash
} 

pub fn human_leftleg_layers() -> Vec<Layer> {
    let mut layerStash = Vec::new();

    let skin = generate_layer("Skin".to_string(), Materials::Skin, 2.0);
    let fat = generate_layer("Fat".to_string(), Materials::Fat, 2.0);
    let muscle = generate_layer("Muscle".to_string(), Materials::Muscle, 1.1);
    let bone = generate_layer("Bone".to_string(), Materials::Bone, 2.0);

    layerStash.push(skin);
    layerStash.push(fat);
    layerStash.push(muscle);
    layerStash.push(bone);

    layerStash
}

pub fn human_rightleg_layers() -> Vec<Layer> {
    let mut layerStash = Vec::new();

    let skin = generate_layer("Skin".to_string(), Materials::Skin, 2.0);
    let fat = generate_layer("Fat".to_string(), Materials::Fat, 2.0);
    let muscle = generate_layer("Muscle".to_string(),Materials::Muscle, 1.1);
    let bone = generate_layer("Bone".to_string(), Materials::Bone, 2.0);

    layerStash.push(skin);
    layerStash.push(fat);
    layerStash.push(muscle);
    layerStash.push(bone);

    layerStash
}