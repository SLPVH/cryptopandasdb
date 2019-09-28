CREATE TYPE physique AS ENUM
    ('standard', 'small', 'slim', 'small face', 'chubby',
    'overweight', 'athletic', 'genius');
	
CREATE TYPE pattern AS ENUM
    ('panda i', 'panda ii', 'panda iii', 'uniform', 'cow',
	'stripes', 'dots', 'bitcoin');
	
CREATE TYPE eye_color AS ENUM
    ('thundergrey', 'gold', 'topaz', 'mintgreen', 'isotope',
    'sizzurp', 'chestnut', 'strawberry', 'sapphire',
    'forgetmenot', 'dahlia', 'coralsunrise', 'olive',
    'doridnudibranch', 'parakeet', 'cyan', 'pumpkin i',
    'limegreen i', 'bridesmaid i', 'bubblegum i',
    'twilightsparkle i', 'palejade i', 'pinefresh i',
    'eclipse I', 'babypuke ii', 'downbythebay ii',
    'autumnmoon ii', 'Oasis ii', 'gemini iii',
    'dioscuri iii', 'kaleidoscope iv', 'unknown');

CREATE TYPE eye_shape AS ENUM
    ('standard', 'small', 'bored', 'wonky',
    'caffeine', 'angry', 'fabulous', 'nerd');
	
CREATE TYPE base_color AS ENUM
    ('shadowgrey', 'salmon', 'meowgarine', 'orangesoda',
    'cottoncandy', 'mauveover', 'aquamarine', 'nachocheez',
    'harbourfog', 'cinderella', 'greymatter', 'tundra',
    'brownies', 'dragonfruit', 'hintomint', 'bananacream',
    'cloudwhite i', 'cornflower i', 'oldlace i', 'koala i',
    'lavender i', 'glacier i', 'redvelvet i', 'verdigris i',
    'icicle ii', 'onyx ii', 'hyacinth ii', 'martian ii',
    'hotcocoa iii', 'shamrock iii', 'firstblush iv',
    'unknown');

CREATE TYPE highlight_color AS ENUM
    ('cyborg', 'springcrocus', 'egyptiankohl',
    'poisonberry','lilac', 'apricot', 'royalpurple',
    'padparadscha', 'swampgreen', 'violet', 'scarlet',
    'barkbrown', 'coffee', 'lemonade', 'chocolate',
    'butterscotch', 'ooze i', 'safetyvest i',
    'turtleback i', 'rosequartz i', 'wolfgrey i', 'cerulian i',
    'skyblue i', 'garnet i', 'peppermint ii', 'universe ii',
    'royalblue ii', 'mertail ii', 'inflatablepool iii',
    'pearl iii', 'prairierose iv', 'unknown');

CREATE TYPE accent_color AS ENUM
    ('belleblue', 'sandalwood', 'peach', 'icy',
    'granitegrey', 'cashewmilk', 'kittencream',
    'emeraldgreen', 'kalahari', 'shale',
    'purplehaze', 'hanauma', 'azaleablush',
    'missmuffett', 'morningglory', 'frosting',
    'daffodil i', 'flamingo i', 'buttercup i',
    'bloodred i', 'atlantis i', 'summerbonnet i',
     'periwinkle i', 'patrickstarfish i', 'seafoam ii',
    'cobalt ii', 'mallowflower ii', 'mintmacaron ii', 
    'sully iii', 'fallspice iii', 'dreamboat iv',
    'unknown');

CREATE TYPE wild_element AS ENUM
    ('standard', 'elk horns', 'third eye', 'icy',
    'bushy tail', 'unicorn');

CREATE TYPE mouth AS ENUM
    ('standard', 'worried', 'happy',
    'oh', 'tongue', 'walrus',
    'nullc', 'amaury');

CREATE TABLE panda
(
    id bigint PRIMARY KEY,
    genesis_tx bigint NOT NULL,
    owner_tx bigint NOT NULL,
    owner_tx_idx bigint NOT NULL,
    physique physique NOT NULL,
    pattern pattern NOT NULL,
    eye_color eye_color NOT NULL,
    eye_shape eye_shape NOT NULL,
    base_color base_color NOT NULL,
    highlight_color highlight_color NOT NULL,
    accent_color accent_color NOT NULL,
    wild_element wild_element NOT NULL,
    mouth mouth NOT NULL,
    FOREIGN KEY (genesis_tx) REFERENCES tx (id) ON DELETE CASCADE,
    FOREIGN KEY (owner_tx, owner_tx_idx) REFERENCES tx_output (tx, idx) ON DELETE CASCADE
);
