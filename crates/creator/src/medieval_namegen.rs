// Medieval Name Generator (inspired by reedsy.com)
// Supports gender, origin, and meaning fields for each name.

#[derive(Debug, Clone)]
pub enum MedievalOrigin {
    OldNorse,
    OldGerman,
    OldRoman,
    OldCeltic,
    OldEnglish,
    Latin,
    Hebrew,
    Other(String),
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Unisex,
}

#[derive(Debug, Clone)]
pub struct MedievalName {
    pub name: String,
    pub meaning: String,
    pub origin: MedievalOrigin,
    pub gender: Gender,
}

pub fn medieval_name_samples() -> Vec<MedievalName> {
    vec![
        MedievalName { name: "Acquilina".into(), meaning: "Eagle".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Female },
        MedievalName { name: "Aileas".into(), meaning: "Noble".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Female },
        MedievalName { name: "Alke".into(), meaning: "Noble sort".into(), origin: MedievalOrigin::OldGerman, gender: Gender::Female },
        MedievalName { name: "Badulf".into(), meaning: "Fight; struggle".into(), origin: MedievalOrigin::OldGerman, gender: Gender::Male },
        MedievalName { name: "Beathan".into(), meaning: "Child of life".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Male },
        MedievalName { name: "Bríd".into(), meaning: "Exalted one".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Female },
        MedievalName { name: "Callicrates".into(), meaning: "Beautiful power".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Male },
        MedievalName { name: "Chrysanta".into(), meaning: "Golden flower".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Female },
        MedievalName { name: "Clayton".into(), meaning: "Clay settlement".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Male },
        MedievalName { name: "Conrad".into(), meaning: "Brave counsel".into(), origin: MedievalOrigin::OldGerman, gender: Gender::Male },
        MedievalName { name: "Cosgrach".into(), meaning: "Victor".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Male },
        MedievalName { name: "Damocles".into(), meaning: "Glory of the people".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Male },
        MedievalName { name: "Earna".into(), meaning: "Eagle".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Female },
        MedievalName { name: "Egil".into(), meaning: "Awe; terror".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Male },
        MedievalName { name: "Eirunn".into(), meaning: "Happiness; gift".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Female },
        MedievalName { name: "Eithne".into(), meaning: "Kernel or grain".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Female },
        MedievalName { name: "Emil".into(), meaning: "Trying to equal".into(), origin: MedievalOrigin::Latin, gender: Gender::Male },
        MedievalName { name: "Eòghann".into(), meaning: "Well-born; youth".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Male },
        MedievalName { name: "Felberta".into(), meaning: "Brilliant".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Female },
        MedievalName { name: "Gage".into(), meaning: "Assayer".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Male },
        MedievalName { name: "Germund".into(), meaning: "Spear".into(), origin: MedievalOrigin::OldGerman, gender: Gender::Male },
        MedievalName { name: "Groa".into(), meaning: "To grow".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Female },
        MedievalName { name: "Havardr".into(), meaning: "High guardian".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Male },
        MedievalName { name: "Haywood".into(), meaning: "Fenced wood".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Male },
        MedievalName { name: "Hella".into(), meaning: "Holy".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Female },
        MedievalName { name: "Ingegerd".into(), meaning: "Enclosure".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Female },
        MedievalName { name: "Innis".into(), meaning: "Island".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Female },
        MedievalName { name: "Ivor".into(), meaning: "Yew bow; warrior".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Male },
        MedievalName { name: "Jorlaug".into(), meaning: "Boar".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Male },
        MedievalName { name: "Jorunn".into(), meaning: "Lover of horses".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Female },
        MedievalName { name: "Kamden".into(), meaning: "Valley".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Male },
        MedievalName { name: "Karena".into(), meaning: "Keel of a ship".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Female },
        MedievalName { name: "Lana".into(), meaning: "Calm as still waters".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Female },
        MedievalName { name: "Levina".into(), meaning: "Bright flash".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Female },
        MedievalName { name: "Lillen".into(), meaning: "Lily".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Female },
        MedievalName { name: "Linn".into(), meaning: "Gentle".into(), origin: MedievalOrigin::OldGerman, gender: Gender::Female },
        MedievalName { name: "Lothar".into(), meaning: "Famous army".into(), origin: MedievalOrigin::OldGerman, gender: Gender::Male },
        MedievalName { name: "Nuala".into(), meaning: "White shoulder".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Female },
        MedievalName { name: "Odilie".into(), meaning: "Wealth".into(), origin: MedievalOrigin::OldGerman, gender: Gender::Female },
        MedievalName { name: "Ovid".into(), meaning: "Sheep herder".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Male },
        MedievalName { name: "Reidun".into(), meaning: "Nest; home".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Male },
        MedievalName { name: "Ròidh".into(), meaning: "Reddish".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Male },
        MedievalName { name: "Runar".into(), meaning: "Secret lore".into(), origin: MedievalOrigin::OldNorse, gender: Gender::Male },
        MedievalName { name: "Scholastica".into(), meaning: "Scholar".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Female },
        MedievalName { name: "Sèitheach".into(), meaning: "Wolf".into(), origin: MedievalOrigin::OldCeltic, gender: Gender::Male },
        MedievalName { name: "Selena".into(), meaning: "Moon".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Female },
        MedievalName { name: "Stephanus".into(), meaning: "Crown".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Male },
        MedievalName { name: "Theophilus".into(), meaning: "Friend of god".into(), origin: MedievalOrigin::OldRoman, gender: Gender::Male },
        MedievalName { name: "Thilde".into(), meaning: "Might; fight".into(), origin: MedievalOrigin::OldGerman, gender: Gender::Female },
        MedievalName { name: "Wynnstan".into(), meaning: "Joy".into(), origin: MedievalOrigin::OldEnglish, gender: Gender::Male },
    ]
}

use rand::seq::SliceRandom;

pub fn random_medieval_name<R: rand::Rng>(rng: &mut R, gender: Option<Gender>, origin: Option<MedievalOrigin>) -> Option<&'static MedievalName> {
    let names = medieval_name_samples();
    let filtered: Vec<&MedievalName> = names.iter()
        .filter(|n| gender.as_ref().map_or(true, |g| &n.gender == g))
        .filter(|n| origin.as_ref().map_or(true, |o| &n.origin == o))
        .collect();
    filtered.choose(rng).copied()
}
