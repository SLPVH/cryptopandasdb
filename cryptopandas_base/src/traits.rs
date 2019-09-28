use std::convert::{TryFrom, TryInto};

use num_enum::TryFromPrimitive;

pub struct GeneticDisorder; // Gene too large

pub trait PandaAttribute
where
    Self: Sized,
{
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder>;
}

pub trait PandaTrait
where
    Self: PandaAttribute,
{
    fn from_gene_slice(genes: &[u8; 4]) -> Result<[Self; 4], GeneticDisorder>;
}

impl<U: PandaAttribute> PandaTrait for U {
    fn from_gene_slice(gene_slice: &[u8; 4]) -> Result<[Self; 4], GeneticDisorder> {
        Ok([
            Self::from_gene(gene_slice[0])?,
            Self::from_gene(gene_slice[1])?,
            Self::from_gene(gene_slice[2])?,
            Self::from_gene(gene_slice[3])?,
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(Physique::try_from(gene / 8).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(Pattern::try_from(gene / 8).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(EyeColor::try_from(gene).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(EyeShape::try_from(gene / 8).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(BaseColor::try_from(gene).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(HighlightColor::try_from(gene).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(AccentColor::try_from(gene).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum WildElement {
    Standard,
    Element,
    ThirdEye,
    BushyTail,
    Unicorn,
}

impl PandaAttribute for WildElement {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            if gene < 16 {
                Ok(WildElement::Standard)
            } else {
                Ok(WildElement::try_from(gene / 8).unwrap())
            }
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
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
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(Mouth::try_from(gene / 8).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Clone, Debug)]
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
    fn from_genes(genes: &[u8; 48]) -> Result<Self, GeneticDisorder> {
        Ok(PandaAttributes {
            physique: Physique::from_gene(genes[0])?,
            pattern: Pattern::from_gene(genes[2])?,
            eye_color: EyeColor::from_gene(genes[5])?,
            eye_shape: EyeShape::from_gene(genes[8])?,
            base_color: BaseColor::from_gene(genes[11])?,
            highlight_color: HighlightColor::from_gene(genes[14])?,
            accent_color: AccentColor::from_gene(genes[17])?,
            wild_element: WildElement::from_gene(genes[20])?,
            mouth: Mouth::from_gene(genes[23])?,
        })
    }
}

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
    fn from_genes(genes: &[u8; 48]) -> Result<Self, GeneticDisorder> {
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
