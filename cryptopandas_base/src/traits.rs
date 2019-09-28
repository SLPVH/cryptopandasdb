use std::convert::{TryFrom, TryInto};

use num_enum::TryFromPrimitive;
use serde::Serialize;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum Physique {
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

impl PandaAttribute for Physique {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(Physique::try_from(gene / 4).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum Pattern {
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

impl PandaAttribute for Pattern {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(Pattern::try_from(gene / 4).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum EyeColor {
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

impl PandaAttribute for EyeColor {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(EyeColor::try_from(gene).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum EyeShape {
    Standard,
    Small,
    Bored,
    Wonky,
    Caffeine,
    Angry,
    Fabulous,
    Nerd,
}

impl PandaAttribute for EyeShape {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(EyeShape::try_from(gene / 4).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum BaseColor {
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

impl PandaAttribute for BaseColor {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(BaseColor::try_from(gene).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum HighlightColor {
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

impl PandaAttribute for HighlightColor {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(HighlightColor::try_from(gene).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum AccentColor {
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

impl PandaAttribute for AccentColor {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(AccentColor::try_from(gene).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum WildElement {
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

impl PandaAttribute for WildElement {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            if gene < 16 {
                Ok(WildElement::Standard)
            } else {
                Ok(WildElement::try_from(gene / 4 - 3).unwrap())
            }
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum Mouth {
    Standard,
    Worried,
    Happy,
    Oh,
    Tongue,
    Walrus,
    Nullc,
    Amaury,
}

impl PandaAttribute for Mouth {
    fn from_gene(gene: u8) -> Result<Self, InvalidGeneInteger> {
        if gene < 32 {
            Ok(Mouth::try_from(gene / 4).unwrap())
        } else {
            Err(InvalidGeneInteger)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PandaAttributes {
    pub physique: Physique,
    pub pattern: Pattern,
    pub eye_color: EyeColor,
    pub eye_shape: EyeShape,
    pub base_color: BaseColor,
    pub highlight_color: HighlightColor,
    pub accent_color: AccentColor,
    pub wild_element: WildElement,
    pub mouth: Mouth,
}

impl PandaAttributes {
    fn from_genes(genes: &[u8; 48]) -> Result<Self, InvalidGeneInteger> {
        Ok(PandaAttributes {
            physique: Physique::from_gene(genes[0])?,
            pattern: Pattern::from_gene(genes[4])?,
            eye_color: EyeColor::from_gene(genes[8])?,
            eye_shape: EyeShape::from_gene(genes[12])?,
            base_color: BaseColor::from_gene(genes[16])?,
            highlight_color: HighlightColor::from_gene(genes[20])?,
            accent_color: AccentColor::from_gene(genes[24])?,
            wild_element: WildElement::from_gene(genes[28])?,
            mouth: Mouth::from_gene(genes[32])?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PandaTraits {
    physique: [Physique; 4],
    pattern: [Pattern; 4],
    eye_color: [EyeColor; 4],
    eye_shape: [EyeShape; 4],
    base_color: [BaseColor; 4],
    highlight_color: [HighlightColor; 4],
    accent_color: [AccentColor; 4],
    wild_element: [WildElement; 4],
    mouth: [Mouth; 4],
}

impl PandaTraits {
    fn from_genes(genes: &[u8; 48]) -> Result<Self, InvalidGeneInteger> {
        Ok(PandaTraits {
            physique: Physique::from_gene_slice(&genes[0..4].try_into().unwrap())?,
            pattern: Pattern::from_gene_slice(&genes[4..8].try_into().unwrap())?,
            eye_color: EyeColor::from_gene_slice(&genes[8..12].try_into().unwrap())?,
            eye_shape: EyeShape::from_gene_slice(&genes[12..16].try_into().unwrap())?,
            base_color: BaseColor::from_gene_slice(&genes[16..20].try_into().unwrap())?,
            highlight_color: HighlightColor::from_gene_slice(&genes[20..24].try_into().unwrap())?,
            accent_color: AccentColor::from_gene_slice(&genes[24..28].try_into().unwrap())?,
            wild_element: WildElement::from_gene_slice(&genes[28..32].try_into().unwrap())?,
            mouth: Mouth::from_gene_slice(&genes[32..36].try_into().unwrap())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wild_elements() {
        for i in 0..16 {
            assert_eq!(WildElement::Standard, WildElement::from_gene(i).unwrap());
        }
        for i in 16..20 {
            assert_eq!(WildElement::ElkHorns, WildElement::from_gene(i).unwrap());
        }
        for i in 20..24 {
            assert_eq!(WildElement::ThirdEye, WildElement::from_gene(i).unwrap());
        }
        for i in 24..28 {
            assert_eq!(WildElement::BushyTail, WildElement::from_gene(i).unwrap());
        }
        for i in 28..32 {
            assert_eq!(WildElement::Unicorn, WildElement::from_gene(i).unwrap());
        }
    }

    #[test]
    fn physique() {
        for i in 0..4 {
            assert_eq!(Physique::Standard, Physique::from_gene(i).unwrap());
        }
        for i in 4..8 {
            assert_eq!(Physique::Small, Physique::from_gene(i).unwrap());
        }
        for i in 8..12 {
            assert_eq!(Physique::Slim, Physique::from_gene(i).unwrap());
        }
        for i in 12..16 {
            assert_eq!(Physique::SmallFace, Physique::from_gene(i).unwrap());
        }
        for i in 16..20 {
            assert_eq!(Physique::Chubby, Physique::from_gene(i).unwrap());
        }
        for i in 20..24 {
            assert_eq!(Physique::Overweight, Physique::from_gene(i).unwrap());
        }
        for i in 24..28 {
            assert_eq!(Physique::Athletic, Physique::from_gene(i).unwrap());
        }
        for i in 28..32 {
            assert_eq!(Physique::Genius, Physique::from_gene(i).unwrap());
        }
    }

    #[test]
    fn sanity_zeros() {
        let zero_array = [0; 48];
        let zero_panda_actual = PandaAttributes::from_genes(&zero_array).unwrap();
        let zero_panda_expected = PandaAttributes {
            physique: Physique::Standard,
            pattern: Pattern::PandaI,
            eye_color: EyeColor::Thundergrey,
            eye_shape: EyeShape::Standard,
            base_color: BaseColor::Shadowgrey,
            highlight_color: HighlightColor::Cyborg,
            accent_color: AccentColor::Belleblue,
            wild_element: WildElement::Standard,
            mouth: Mouth::Standard,
        };
        assert_eq!(zero_panda_actual, zero_panda_expected);
    }

    #[test]
    fn sanity_max() {
        let max_array = [31; 48];
        let max_panda_actual = PandaAttributes::from_genes(&max_array).unwrap();
        let max_panda_expected = PandaAttributes {
            physique: Physique::Genius,
            pattern: Pattern::Bitcoin,
            eye_color: EyeColor::Unknown,
            eye_shape: EyeShape::Nerd,
            base_color: BaseColor::Unknown,
            highlight_color: HighlightColor::Unknown,
            accent_color: AccentColor::Unknown,
            wild_element: WildElement::Unicorn,
            mouth: Mouth::Amaury,
        };
        assert_eq!(max_panda_actual, max_panda_expected);
    }

}
