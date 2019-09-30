use std::convert::{TryFrom, TryInto};

use bitvec::prelude::*;
use diesel_derive_enum::DbEnum;
use num_enum::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct InvalidGeneInteger; // Gene too large

pub trait PandaAttribute
where
    Self: Sized,
{
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger>;
}

pub trait PandaTrait
where
    Self: PandaAttribute,
{
    fn from_gene_slice(genes: &[u8; 4]) -> Result<[Self; 4], InvalidGeneInteger>;
}

impl<U: PandaAttribute> PandaTrait for U {
    fn from_gene_slice(gene_slice: &[u8; 4]) -> Result<[Self; 4], InvalidGeneInteger> {
        Ok([
            Self::from_gene(gene_slice[0])?,
            Self::from_gene(gene_slice[1])?,
            Self::from_gene(gene_slice[2])?,
            Self::from_gene(gene_slice[3])?,
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive, IntoPrimitive, DbEnum)]
#[DieselType = "Physique"]
#[repr(u8)]
pub enum PhysiqueTrait {
    Standard, // Default is a reversed word
    Small,
    Slim,
    #[serde(rename = "Small Face")]
    SmallFace,
    Chubby,
    Overweight,
    Athletic,
    Genius,
}

impl PandaAttribute for PhysiqueTrait {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(PhysiqueTrait::try_from(gene / 4).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive, IntoPrimitive, DbEnum)]
#[DieselType = "Pattern"]
#[repr(u8)]
pub enum PatternTrait {
    #[serde(rename = "Panda I")]
    PandaI,
    #[serde(rename = "Panda II")]
    PandaII,
    #[serde(rename = "Panda III")]
    PandaIII,
    Uniform,
    Cow,
    Stripes,
    Dots,
    Bitcoin,
}

impl PandaAttribute for PatternTrait {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(PatternTrait::try_from(gene / 4).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive, IntoPrimitive, DbEnum)]
#[DieselType = "Eye_color"]
#[repr(u8)]
pub enum EyeColorTrait {
    Thundergrey,
    Gold,
    Topaz,
    Mintgreen,
    Isotope,
    Sizzurp,
    Chestnut,
    Strawberry,
    Sapphire,
    Forgetmenot,
    Dahlia,
    Coralsunrise,
    Olive,
    Doridnudibranch,
    Parakeet,
    Cyan,
    #[serde(rename = "Pumpkin I")]
    PumpkinI,
    #[serde(rename = "Limegreen I")]
    LimegreenI,
    #[serde(rename = "Bridesmaid I")]
    BridesmaidI,
    #[serde(rename = "Bubblegum I")]
    BubblegumI,
    #[serde(rename = "Twilightsparkle I")]
    TwilightsparkleI,
    #[serde(rename = "Palejade I")]
    PalejadeI,
    #[serde(rename = "Pinefresh I")]
    PinefreshI,
    #[serde(rename = "Eclipse I")]
    EclipseI,
    #[serde(rename = "Babypuke II")]
    BabypukeII,
    #[serde(rename = "Downbythebay II")]
    DownbythebayII,
    #[serde(rename = "Autumnmoon II")]
    AutumnmoonII,
    #[serde(rename = "Oasis II")]
    OasisII,
    #[serde(rename = "Gemini III")]
    GeminiIII,
    #[serde(rename = "Dioscuri III")]
    DioscuriIII,
    #[serde(rename = "Kaleidoscope IV")]
    KaleidoscopeIV,
    #[serde(rename = "???")]
    Unknown,
}

impl PandaAttribute for EyeColorTrait {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(EyeColorTrait::try_from(gene).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive, IntoPrimitive, DbEnum)]
#[DieselType = "Eye_shape"]
#[repr(u8)]
pub enum EyeShapeTrait {
    Standard,
    Small,
    Bored,
    Wonky,
    Caffeine,
    Angry,
    Fabulous,
    Nerd,
}

impl PandaAttribute for EyeShapeTrait {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(EyeShapeTrait::try_from(gene / 4).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive, IntoPrimitive, DbEnum)]
#[DieselType = "Base_color"]
#[repr(u8)]
pub enum BaseColorTrait {
    Shadowgrey,
    Salmon,
    Meowgarine,
    Orangesoda,
    Cottoncandy,
    Mauveover,
    Aquamarine,
    Nachocheez,
    Harbourfog,
    Cinderella,
    Greymatter,
    Tundra,
    Brownies,
    Dragonfruit,
    Hintomint,
    Bananacream,
    #[serde(rename = "Cloudwhite I")]
    CloudwhiteI,
    #[serde(rename = "Cornflower I")]
    CornflowerI,
    #[serde(rename = "Oldlace I")]
    OldlaceI,
    #[serde(rename = "Koala I")]
    KoalaI,
    #[serde(rename = "Lavender I")]
    LavenderI,
    #[serde(rename = "Glacier I")]
    GlacierI,
    #[serde(rename = "Redvelvet I")]
    RedvelvetI,
    #[serde(rename = "Verdigris I")]
    VerdigrisI,
    #[serde(rename = "Icicle II")]
    IcicleII,
    #[serde(rename = "Onyx II")]
    OnyxII,
    #[serde(rename = "Hyacinth II")]
    HyacinthII,
    #[serde(rename = "Martian II")]
    MartianII,
    #[serde(rename = "Hotcocoa III")]
    HotcocoaIII,
    #[serde(rename = "Shamrock III")]
    ShamrockIII,
    #[serde(rename = "Firstblush IV")]
    FirstblushIV,
    #[serde(rename = "???")]
    Unknown,
}

impl PandaAttribute for BaseColorTrait {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(BaseColorTrait::try_from(gene).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive, IntoPrimitive, DbEnum)]
#[DieselType = "Highlight_color"]
#[repr(u8)]
pub enum HighlightColorTrait {
    Cyborg,
    Springcrocus,
    Egyptiankohl,
    Poisonberry,
    Lilac,
    Apricot,
    Royalpurple,
    Padparadscha,
    Swampgreen,
    Violet,
    Scarlet,
    Barkbrown,
    Coffee,
    Lemonade,
    Chocolate,
    Butterscotch,
    #[serde(rename = "Ooze I")]
    OozeI,
    #[serde(rename = "Safetyvest I")]
    SafetyvestI,
    #[serde(rename = "Turtleback I")]
    TurtlebackI,
    #[serde(rename = "Rosequartz I")]
    RosequartzI,
    #[serde(rename = "Wolfgrey I")]
    WolfgreyI,
    #[serde(rename = "Cerulian I")]
    CerulianI,
    #[serde(rename = "Skyblue I")]
    SkyblueI,
    #[serde(rename = "Garnet I")]
    GarnetI,
    #[serde(rename = "Peppermint II")]
    PeppermintII,
    #[serde(rename = "Universe II")]
    UniverseII,
    #[serde(rename = "Royalblue II")]
    RoyalblueII,
    #[serde(rename = "Mertail II")]
    MertailII,
    #[serde(rename = "Inflatablepool III")]
    InflatablepoolIII,
    #[serde(rename = "Pearl III")]
    PearlIII,
    #[serde(rename = "Prairierose IV")]
    PrairieroseIV,
    #[serde(rename = "???")]
    Unknown,
}

impl PandaAttribute for HighlightColorTrait {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(HighlightColorTrait::try_from(gene).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive, IntoPrimitive, DbEnum)]
#[DieselType = "Accent_color"]
#[repr(u8)]
pub enum AccentColorTrait {
    Belleblue,
    Sandalwood,
    Peach,
    Icy,
    Granitegrey,
    Cashewmilk,
    Kittencream,
    Emeraldgreen,
    Kalahari,
    Shale,
    Purplehaze,
    Hanauma,
    Azaleablush,
    Missmuffett,
    Morningglory,
    Frosting,
    #[serde(rename = "Daffodil I")]
    DaffodilI,
    #[serde(rename = "Flamingo I")]
    FlamingoI,
    #[serde(rename = "Buttercup I")]
    ButtercupI,
    #[serde(rename = "Bloodred I")]
    BloodredI,
    #[serde(rename = "Atlantis I")]
    AtlantisI,
    #[serde(rename = "Summerbonnet I")]
    SummerbonnetI,
    #[serde(rename = "Periwinkle I")]
    PeriwinkleI,
    #[serde(rename = "Patrickstarfish I")]
    PatrickstarfishI,
    #[serde(rename = "Seafoam II")]
    SeafoamII,
    #[serde(rename = "Cobalt II")]
    CobaltII,
    #[serde(rename = "Mallowflower II")]
    MallowflowerII,
    #[serde(rename = "Mintmacaron II")]
    MintmacaronII,
    #[serde(rename = "Sully III")]
    SullyIII,
    #[serde(rename = "Fallspice III")]
    FallspiceIII,
    #[serde(rename = "Dreamboat IV")]
    DreamboatIV,
    #[serde(rename = "???")]
    Unknown,
}

impl PandaAttribute for AccentColorTrait {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(AccentColorTrait::try_from(gene).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive, IntoPrimitive, DbEnum)]
#[DieselType = "Wild_element"]
#[repr(u8)]
pub enum WildElementTrait {
    #[serde(rename = "None")]
    Standard,
    #[serde(rename = "Elk Horns")]
    ElkHorns,
    #[serde(rename = "Third Eye")]
    ThirdEye,
    #[serde(rename = "Bushy Tail")]
    BushyTail,
    Unicorn,
}

impl PandaAttribute for WildElementTrait {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            if gene < 16 {
                Ok(WildElementTrait::Standard)
            } else {
                Ok(WildElementTrait::try_from(gene / 4 - 3).unwrap())
            }
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    TryFromPrimitive,
    IntoPrimitive,
    DbEnum,
)]
#[DieselType = "Mouth"]
#[repr(u8)]
pub enum MouthTrait {
    Standard,
    Worried,
    Happy,
    Oh,
    Tongue,
    Walrus,
    Nullc,
    Amaury,
}

impl PandaAttribute for MouthTrait {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(MouthTrait::try_from(gene / 4).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PandaAttributes {
    pub physique: PhysiqueTrait,
    pub pattern: PatternTrait,
    pub eye_color: EyeColorTrait,
    pub eye_shape: EyeShapeTrait,
    pub base_color: BaseColorTrait,
    pub highlight_color: HighlightColorTrait,
    pub accent_color: AccentColorTrait,
    pub wild_element: WildElementTrait,
    pub mouth: MouthTrait,
}

impl PandaAttributes {
    pub fn from_genes(genes: &[u8; 48]) -> Result<Self, InvalidGeneInteger> {
        Ok(PandaAttributes {
            physique: PhysiqueTrait::from_gene(genes[0])?,
            pattern: PatternTrait::from_gene(genes[4])?,
            eye_color: EyeColorTrait::from_gene(genes[8])?,
            eye_shape: EyeShapeTrait::from_gene(genes[12])?,
            base_color: BaseColorTrait::from_gene(genes[16])?,
            highlight_color: HighlightColorTrait::from_gene(genes[20])?,
            accent_color: AccentColorTrait::from_gene(genes[24])?,
            wild_element: WildElementTrait::from_gene(genes[28])?,
            mouth: MouthTrait::from_gene(genes[32])?,
        })
    }

    pub fn to_base32(&self) -> String {
        let alph = base32::Alphabet::RFC4648 { padding: false };
        let trait_slice: [u8; 9] = [
            self.physique.into(),
            self.pattern.into(),
            self.eye_color.into(),
            self.eye_shape.into(),
            self.base_color.into(),
            self.highlight_color.into(),
            self.accent_color.into(),
            self.wild_element.into(),
            self.mouth.into(),
        ];
        trait_slice
            .iter()
            .map(|trait_int| base32::encode(alph, &[*trait_int]).chars().next().unwrap())
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PandaTraits {
    physique: [PhysiqueTrait; 4],
    pattern: [PatternTrait; 4],
    eye_color: [EyeColorTrait; 4],
    eye_shape: [EyeShapeTrait; 4],
    base_color: [BaseColorTrait; 4],
    highlight_color: [HighlightColorTrait; 4],
    accent_color: [AccentColorTrait; 4],
    wild_element: [WildElementTrait; 4],
    mouth: [MouthTrait; 4],
}

impl PandaTraits {
    pub fn from_genes(genes: &[u8; 48]) -> Result<Self, InvalidGeneInteger> {
        Ok(PandaTraits {
            physique: PhysiqueTrait::from_gene_slice(&genes[0..4].try_into().unwrap())?,
            pattern: PatternTrait::from_gene_slice(&genes[4..8].try_into().unwrap())?,
            eye_color: EyeColorTrait::from_gene_slice(&genes[8..12].try_into().unwrap())?,
            eye_shape: EyeShapeTrait::from_gene_slice(&genes[12..16].try_into().unwrap())?,
            base_color: BaseColorTrait::from_gene_slice(&genes[16..20].try_into().unwrap())?,
            highlight_color: HighlightColorTrait::from_gene_slice(
                &genes[20..24].try_into().unwrap(),
            )?,
            accent_color: AccentColorTrait::from_gene_slice(&genes[24..28].try_into().unwrap())?,
            wild_element: WildElementTrait::from_gene_slice(&genes[28..32].try_into().unwrap())?,
            mouth: MouthTrait::from_gene_slice(&genes[32..36].try_into().unwrap())?,
        })
    }

    pub fn to_attributes(&self) -> PandaAttributes {
        PandaAttributes {
            physique: self.physique[0],
            pattern: self.pattern[0],
            eye_color: self.eye_color[0],
            eye_shape: self.eye_shape[0],
            base_color: self.base_color[0],
            highlight_color: self.highlight_color[0],
            accent_color: self.accent_color[0],
            wild_element: self.wild_element[0],
            mouth: self.mouth[0],
        }
    }

    pub fn to_byte_public_genes(&self) -> [u8; 36] {
        let mut genes: [u8; 36] = [0; 36];
        for (i, physique) in self.physique.iter().cloned().enumerate() {
            genes[i] = physique.into();
        }
        for (i, pattern) in self.pattern.iter().cloned().enumerate() {
            genes[4 + i] = pattern.into();
        }
        for (i, eye_color) in self.eye_color.iter().cloned().enumerate() {
            genes[8 + i] = eye_color.into();
        }
        for (i, eye_shape) in self.eye_shape.iter().cloned().enumerate() {
            genes[12 + i] = eye_shape.into();
        }
        for (i, base_color) in self.base_color.iter().cloned().enumerate() {
            genes[16 + i] = base_color.into();
        }
        for (i, highlight_color) in self.highlight_color.iter().cloned().enumerate() {
            genes[20 + i] = highlight_color.into();
        }
        for (i, accent_color) in self.accent_color.iter().cloned().enumerate() {
            genes[24 + i] = accent_color.into();
        }
        for (i, wild_element) in self.wild_element.iter().cloned().enumerate() {
            genes[28 + i] = wild_element.into();
        }
        for (i, mouth) in self.mouth.iter().cloned().enumerate() {
            genes[32 + i] = mouth.into();
        }
        genes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base32() {
        let zero_panda_expected = PandaAttributes {
            physique: PhysiqueTrait::Standard,
            pattern: PatternTrait::PandaI,
            eye_color: EyeColorTrait::Thundergrey,
            eye_shape: EyeShapeTrait::Standard,
            base_color: BaseColorTrait::Shadowgrey,
            highlight_color: HighlightColorTrait::Cyborg,
            accent_color: AccentColorTrait::Belleblue,
            wild_element: WildElementTrait::Standard,
            mouth: MouthTrait::Standard,
        };
        assert_eq!("AAAAAAAAA".to_string(), zero_panda_expected.to_base32())
    }

    #[test]
    fn wild_elements() {
        for i in 0..16 {
            assert_eq!(
                WildElementTrait::Standard,
                WildElementTrait::from_gene(i).unwrap()
            );
        }
        for i in 16..20 {
            assert_eq!(
                WildElementTrait::ElkHorns,
                WildElementTrait::from_gene(i).unwrap()
            );
        }
        for i in 20..24 {
            assert_eq!(
                WildElementTrait::ThirdEye,
                WildElementTrait::from_gene(i).unwrap()
            );
        }
        for i in 24..28 {
            assert_eq!(
                WildElementTrait::BushyTail,
                WildElementTrait::from_gene(i).unwrap()
            );
        }
        for i in 28..32 {
            assert_eq!(
                WildElementTrait::Unicorn,
                WildElementTrait::from_gene(i).unwrap()
            );
        }
    }

    #[test]
    fn physique() {
        for i in 0..4 {
            assert_eq!(
                PhysiqueTrait::Standard,
                PhysiqueTrait::from_gene(i).unwrap()
            );
        }
        for i in 4..8 {
            assert_eq!(PhysiqueTrait::Small, PhysiqueTrait::from_gene(i).unwrap());
        }
        for i in 8..12 {
            assert_eq!(PhysiqueTrait::Slim, PhysiqueTrait::from_gene(i).unwrap());
        }
        for i in 12..16 {
            assert_eq!(
                PhysiqueTrait::SmallFace,
                PhysiqueTrait::from_gene(i).unwrap()
            );
        }
        for i in 16..20 {
            assert_eq!(PhysiqueTrait::Chubby, PhysiqueTrait::from_gene(i).unwrap());
        }
        for i in 20..24 {
            assert_eq!(
                PhysiqueTrait::Overweight,
                PhysiqueTrait::from_gene(i).unwrap()
            );
        }
        for i in 24..28 {
            assert_eq!(
                PhysiqueTrait::Athletic,
                PhysiqueTrait::from_gene(i).unwrap()
            );
        }
        for i in 28..32 {
            assert_eq!(PhysiqueTrait::Genius, PhysiqueTrait::from_gene(i).unwrap());
        }
    }

    #[test]
    fn sanity_zeros() {
        let zero_array = [0; 48];
        let zero_panda_actual = PandaAttributes::from_genes(&zero_array).unwrap();
        let zero_panda_expected = PandaAttributes {
            physique: PhysiqueTrait::Standard,
            pattern: PatternTrait::PandaI,
            eye_color: EyeColorTrait::Thundergrey,
            eye_shape: EyeShapeTrait::Standard,
            base_color: BaseColorTrait::Shadowgrey,
            highlight_color: HighlightColorTrait::Cyborg,
            accent_color: AccentColorTrait::Belleblue,
            wild_element: WildElementTrait::Standard,
            mouth: MouthTrait::Standard,
        };
        assert_eq!(zero_panda_actual, zero_panda_expected);
    }

    #[test]
    fn sanity_max() {
        let max_array = [31; 48];
        let max_panda_actual = PandaAttributes::from_genes(&max_array).unwrap();
        let max_panda_expected = PandaAttributes {
            physique: PhysiqueTrait::Genius,
            pattern: PatternTrait::Bitcoin,
            eye_color: EyeColorTrait::Unknown,
            eye_shape: EyeShapeTrait::Nerd,
            base_color: BaseColorTrait::Unknown,
            highlight_color: HighlightColorTrait::Unknown,
            accent_color: AccentColorTrait::Unknown,
            wild_element: WildElementTrait::Unicorn,
            mouth: MouthTrait::Amaury,
        };
        assert_eq!(max_panda_actual, max_panda_expected);
    }

    #[test]
    fn byte_genes_conversion() {
        let panda_traits_expected = PandaTraits {
            physique: [PhysiqueTrait::Standard; 4],
            pattern: [PatternTrait::PandaI; 4],
            eye_color: [EyeColorTrait::Thundergrey; 4],
            eye_shape: [EyeShapeTrait::Standard; 4],
            base_color: [BaseColorTrait::Shadowgrey; 4],
            highlight_color: [HighlightColorTrait::Cyborg; 4],
            accent_color: [AccentColorTrait::Belleblue; 4],
            wild_element: [WildElementTrait::Standard; 4],
            mouth: [MouthTrait::Standard; 4],
        };
        let public_genes = panda_traits_expected.to_byte_public_genes();
        // Extend short genes
        let genes_extended = [0; 12];
        let genes_full_vec = &[&public_genes[..], &genes_extended[..]].concat();
        let mut genes_full = [0; 48];
        genes_full.copy_from_slice(genes_full_vec);
        let panda_traits_actual = PandaTraits::from_genes(&genes_full).unwrap();
        assert_eq!(panda_traits_expected, panda_traits_actual)
    }
}
