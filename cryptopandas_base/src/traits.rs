use std::convert::{TryFrom, TryInto};

use num_enum::TryFromPrimitive;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Physique {
    Standard, // Default is a reversed word
    Small,
    Slim,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Pattern {
    PandaI,
    PandaII,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    PumpkinOne,
    LimegreenOne,
    BridesmaidOne,
    BubblegumI,
    TwilightsparkleI,
    PalejadeI,
    PinefreshI,
    EclipseI,
    BabypukeII,
    DownbythebayII,
    AutumnmoonII,
    OasisII,
    GeminiIII,
    DioscuriIII,
    KaleidoscopeIIII,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    CloudwhiteI,
    CornflowerI,
    OldlaceI,
    KoalaI,
    LavenderI,
    GlacierI,
    RedvelvetI,
    VerdigrisI,
    IcicleII,
    OnyxII,
    HyacinthII,
    MartianII,
    HotcocoaIII,
    ShamrockIII,
    FirstblushIIII,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    OozeI,
    SafetyvestI,
    TurtlebackI,
    RosequartzI,
    WolfgreyI,
    CerulianI,
    SkyblueI,
    GarnetI,
    PeppermintII,
    UniverseII,
    RoyalblueII,
    MertailII,
    InflatablepoolIII,
    PearlIII,
    PrairieroseIIII,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    DaffodilI,
    FlamingoI,
    ButtercupI,
    BloodredI,
    AtlantisI,
    SummerbonnetI,
    PeriwinkleI,
    PatrickstarfishI,
    SeafoamII,
    CobaltII,
    MallowflowerII,
    MintmacaronII,
    SullyIII,
    FallspiceIII,
    DreamboatIIII,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum WildElement {
    Standard,
    ElkHorns,
    ThirdEye,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PandaAttributes {
    physique: Physique,
    pattern: Pattern,
    eye_color: EyeColor,
    eye_shape: EyeShape,
    base_color: BaseColor,
    highlight_color: HighlightColor,
    accent_color: AccentColor,
    wild_element: WildElement,
    mouth: Mouth,
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
