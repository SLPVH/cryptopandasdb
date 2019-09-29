CREATE TYPE physique AS ENUM
    ('standard', 'small', 'slim', 'small_face', 'chubby',
    'overweight', 'athletic', 'genius');
	
CREATE TYPE pattern AS ENUM
    ('panda_i', 'panda_ii', 'panda_iii', 'uniform', 'cow',
	'stripes', 'dots', 'bitcoin');
	
CREATE TYPE eye_color AS ENUM
    ('thundergrey', 'gold', 'topaz', 'mintgreen', 'isotope',
    'sizzurp', 'chestnut', 'strawberry', 'sapphire',
    'forgetmenot', 'dahlia', 'coralsunrise', 'olive',
    'doridnudibranch', 'parakeet', 'cyan', 'pumpkin_i',
    'limegreen_i', 'bridesmaid_i', 'bubblegum_i',
    'twilightsparkle_i', 'palejade_i', 'pinefresh_i',
    'eclipse_i', 'babypuke_ii', 'downbythebay_ii',
    'autumnmoon_ii', 'oasis_ii', 'gemini_iii',
    'dioscuri_iii', 'kaleidoscope_iv', 'unknown');

CREATE TYPE eye_shape AS ENUM
    ('standard', 'small', 'bored', 'wonky',
    'caffeine', 'angry', 'fabulous', 'nerd');
	
CREATE TYPE base_color AS ENUM
    ('shadowgrey', 'salmon', 'meowgarine', 'orangesoda',
    'cottoncandy', 'mauveover', 'aquamarine', 'nachocheez',
    'harbourfog', 'cinderella', 'greymatter', 'tundra',
    'brownies', 'dragonfruit', 'hintomint', 'bananacream',
    'cloudwhite_i', 'cornflower_i', 'oldlace_i', 'koala_i',
    'lavender_i', 'glacier_i', 'redvelvet_i', 'verdigris_i',
    'icicle_ii', 'onyx_ii', 'hyacinth_ii', 'martian_ii',
    'hotcocoa_iii', 'shamrock_iii', 'firstblush_iv',
    'unknown');

CREATE TYPE highlight_color AS ENUM
    ('cyborg', 'springcrocus', 'egyptiankohl',
    'poisonberry','lilac', 'apricot', 'royalpurple',
    'padparadscha', 'swampgreen', 'violet', 'scarlet',
    'barkbrown', 'coffee', 'lemonade', 'chocolate',
    'butterscotch', 'ooze_i', 'safetyvest_i',
    'turtleback_i', 'rosequartz_i', 'wolfgrey_i', 'cerulian_i',
    'skyblue_i', 'garnet_i', 'peppermint_ii', 'universe_ii',
    'royalblue_ii', 'mertail_ii', 'inflatablepool_iii',
    'pearl_iii', 'prairierose_iv', 'unknown');

CREATE TYPE accent_color AS ENUM
    ('belleblue', 'sandalwood', 'peach', 'icy',
    'granitegrey', 'cashewmilk', 'kittencream',
    'emeraldgreen', 'kalahari', 'shale',
    'purplehaze', 'hanauma', 'azaleablush',
    'missmuffett', 'morningglory', 'frosting',
    'daffodil_i', 'flamingo_i', 'buttercup_i',
    'bloodred_i', 'atlantis_i', 'summerbonnet_i',
    'periwinkle_i', 'patrickstarfish_i', 'seafoam_ii',
    'cobalt_ii', 'mallowflower_ii', 'mintmacaron_ii', 
    'sully_iii', 'fallspice_iii', 'dreamboat_iv',
    'unknown');

CREATE TYPE wild_element AS ENUM
    ('standard', 'elk_horns', 'third_eye', 'icy',
    'bushy_tail', 'unicorn');

CREATE TYPE mouth AS ENUM
    ('standard', 'worried', 'happy',
    'oh', 'tongue', 'walrus',
    'nullc', 'amaury');

CREATE TABLE panda
(
    id BIGSERIAL PRIMARY KEY,
    genesis_tx bigint NOT NULL,
    owner_tx bigint NOT NULL,
    owner_tx_idx int NOT NULL,
    physique physique NOT NULL,
    pattern pattern NOT NULL,
    eye_color eye_color NOT NULL,
    eye_shape eye_shape NOT NULL,
    base_color base_color NOT NULL,
    highlight_color highlight_color NOT NULL,
    accent_color accent_color NOT NULL,
    wild_element wild_element NOT NULL,
    mouth mouth NOT NULL,
    genes bytea NOT NULL,
    FOREIGN KEY (genesis_tx) REFERENCES tx (id) ON DELETE CASCADE,
    FOREIGN KEY (owner_tx, owner_tx_idx) REFERENCES tx_output (tx, idx) ON DELETE CASCADE
);
