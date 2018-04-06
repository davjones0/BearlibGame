use Materials;

pub mod layermap;
use self::layermap::Layer;

type BodyPartId = &'static str;


#[derive(Copy, Clone, Debug)]
pub enum damageType {
    None,
    bruise,
    slice,
    pierce,
    shot,
    burn,
    freeze,
    shock,
    dent(i32),
}

#[derive(Clone, Debug)]
pub enum tags {
    breathe,
    vision,
    hearing,
    circulation,
    grasp,
    gaugable,
    thought,
    external,
    weapon,
    identifier(String),
    blood_filter,
    light_bleeding,
    heavy_bleeding,
    resist_poison,
    fallout,
    paralyse_lowerbody,
    paralyse_body,
    embedded,
    internal,
    small,
    position(Position),
    skeleton,
}

#[derive(Clone, Debug)]
pub enum Position {
    front,
    back,
    left,
    right,
}


pub enum organs {
    heart,
    lung,
    stomach,
    kidney,
    intestines,
    horn,
    lower_spine,
    upper_spine,
    eye
}


#[derive(Clone, Debug)]
pub struct BodyPart {
    Id: BodyPartId,
    Name: String,
    Layers: Option<Vec<Layer>>,
    Material: Materials,
    Connects: BodyPartId,
    Core: bool,
    Effects: Vec<tags>,
    Injury: damageType,
}

impl BodyPart {
    pub fn new(id: BodyPartId, name: String, layers: Option<Vec<Layer>>, material: Materials, connects: BodyPartId, core: bool, effects: Vec<tags>, injury: damageType) -> BodyPart {
        BodyPart {
            Id: id,
            Name: name,
            Layers: layers,
            Material: material,
            Connects: connects,
            Core: core,
            Effects: effects,
            Injury: injury,
        }
    }

    pub fn get_Id(&self) -> BodyPartId {
        self.Id
    }

    pub fn get_Name(&self) -> &str {
        &self.Name
    }

    pub fn get_Layers(&self) -> &Option<Vec<Layer>>{
        &self.Layers
    }

}


fn normal_heart() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::circulation);
    tag_list.push(tags::heavy_bleeding);
    tag_list.push(tags::internal);
    tag_list.push(tags::small);
    BodyPart::new("HRT", "Heart".to_string(), None, Materials::Tissue, "UPR", false, tag_list, damageType::None)
}

fn normal_right_lung() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::breathe);
    tag_list.push(tags::light_bleeding);
    tag_list.push(tags::small);
    tag_list.push(tags::identifier("right".to_string()));
    BodyPart::new("RLNG", "Lung".to_string(), None, Materials::Tissue, "UPR", false, tag_list, damageType::None)
}

fn normal_left_lung() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::breathe);
    tag_list.push(tags::light_bleeding);
    tag_list.push(tags::small);
    tag_list.push(tags::identifier("left".to_string()));
    BodyPart::new("LLNG", "Lung".to_string(), None, Materials::Tissue, "UPR", false, tag_list, damageType::None)
}

fn normal_stomach() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::heavy_bleeding);
    tag_list.push(tags::internal);
    tag_list.push(tags::small);
    BodyPart::new("STM", "Stomach".to_string(), None, Materials::Tissue, "LWR", false, tag_list, damageType::None)
}

fn normal_right_kidney() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::heavy_bleeding);
    tag_list.push(tags::blood_filter);
    tag_list.push(tags::small);
    tag_list.push(tags::identifier("right".to_string()));
    BodyPart::new("RKD", "Kidney".to_string(), None, Materials::Tissue, "LWR", false, tag_list, damageType::None)
}

fn normal_left_kidney() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::heavy_bleeding);
    tag_list.push(tags::blood_filter);
    tag_list.push(tags::internal);
    tag_list.push(tags::small);
    tag_list.push(tags::identifier("left".to_string()));
    BodyPart::new("LKD", "Kidney".to_string(), None, Materials::Tissue, "LWR", false, tag_list, damageType::None)
}

fn normal_intestines() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::heavy_bleeding);
    tag_list.push(tags::fallout);
    BodyPart::new("INT" ,"Intestines".to_string(), None, Materials::Tissue, "LWR", false, tag_list, damageType::None)
}

fn normal_horn() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::external);
    tag_list.push(tags::weapon);
    BodyPart::new("HRN", "Horn".to_string(), None, Materials::Tissue, "HD", false, tag_list, damageType::None)
}

fn normal_lower_spin() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::paralyse_lowerbody);
    tag_list.push(tags::position(Position::back));
    tag_list.push(tags::skeleton);
    tag_list.push(tags::internal);
    BodyPart::new("LRSP", "Lower Spine".to_string(), None, Materials::Bone, "LWR", false, tag_list, damageType::None)
}

fn normal_upper_spine() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::paralyse_body);
    tag_list.push(tags::position(Position::back));
    tag_list.push(tags::skeleton);
    tag_list.push(tags::internal);
    BodyPart::new("URSP", "Upper Spine".to_string(), None, Materials::Bone, "UPR", false, tag_list, damageType::None)
}

fn normal_right_eye() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::vision);
    tag_list.push(tags::embedded);
    tag_list.push(tags::identifier("right".to_string()));
    BodyPart::new("REY", "Eye".to_string(), None, Materials::Tissue, "HD", false, tag_list, damageType::None)
}

fn normal_left_eye() -> BodyPart {
    let mut tag_list = Vec::new();
    tag_list.push(tags::vision);
    tag_list.push(tags::embedded);
    tag_list.push(tags::identifier("left".to_string()));
    BodyPart::new("LEY", "Eye".to_string(), None, Materials::Tissue, "HD", false, tag_list, damageType::None)
}

pub fn fetch_bodypart(org: organs) -> BodyPart {
    match org {
        organs::heart => normal_heart(),
        organs::lung => normal_left_lung(),
        organs::horn => normal_horn(),
        organs::intestines => normal_intestines(),
        organs::kidney => normal_left_kidney(),
        organs::lower_spine => normal_lower_spin(),
        organs::stomach => normal_stomach(),
        organs::upper_spine => normal_upper_spine(),
        organs::eye => normal_left_eye()
    }
}
