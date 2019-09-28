use std::convert::TryFrom;

use num_enum::TryFromPrimitive;

pub struct GeneticDisorder; // Gene too large

pub trait GeneticTrait
where Self: Sized {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder>;
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Physique {
    Standard, // Default is a reversed word
    Small,
    Slim,
    SmallFace,
    Chubby,
    Overweight,
    Athletic,
    Genius
}

impl GeneticTrait for Physique {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(Physique::try_from(gene / 8).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Pattern {
    PandaI,
    PandaII,
    PandaIII,
    Uniform,
    Cow,
    Stripes,
    Dots,
    Bitcoin
}

impl GeneticTrait for Pattern {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(Pattern::try_from(gene / 8).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
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
    Unknown
}

impl GeneticTrait for EyeColor {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(EyeColor::try_from(gene).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum EyeShape {
    Standard,
    Small,
    Bored,
    Wonky,
    Caffeine,
    Angry,
    Fabulous,
    Nerd
}

impl GeneticTrait for EyeShape {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(EyeShape::try_from(gene / 8).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
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
    Unknown
}

impl GeneticTrait for BaseColor {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(BaseColor::try_from(gene).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
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
    Unknown
}

impl GeneticTrait for HighlightColor {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(HighlightColor::try_from(gene).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
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
    Unknown
}

impl GeneticTrait for AccentColor {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(AccentColor::try_from(gene).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum WildElement {
    Standard,
    Element,
    ThirdEye,
    BushyTail,
    Unicorn
}

impl GeneticTrait for WildElement {
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

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Mouth {
    Standard,
    Worried,
    Happy,
    Oh,
    Tongue,
    Walrus,
    Nullc,
    Amaury
}

impl GeneticTrait for Mouth {
    fn from_gene(gene: u8) -> Result<Self, GeneticDisorder> {
        if gene < 32 {
            Ok(Mouth::try_from(gene / 8).unwrap())
        } else {
            Err(GeneticDisorder)
        }
    }
}

pub struct PandaTraits {
    physique: Physique,
    pattern: Pattern,
    eye_color: EyeColor,
    eye_shape: EyeShape,
    base_color: BaseColor,
    highlight_color: HighlightColor,
    accent_color: AccentColor,
    wild_element: WildElement,
    mouth: Mouth
}

