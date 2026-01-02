use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Output format", default_value_t = Format::Normal, value_parser = clap::value_parser!(Format))]
    format: Format,
    #[arg(short, long, help = "Number of names to generate", default_value_t = 1)]
    count: u32,
    #[arg(long, help = "Copy first name to clipboard")]
    copy: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Normal,
    Kebab,
    Snake,
    Constant,
    Camel,
    Pascal,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Normal => write!(f, "normal"),
            Format::Kebab => write!(f, "kebab"),
            Format::Snake => write!(f, "snake"),
            Format::Constant => write!(f, "constant"),
            Format::Camel => write!(f, "camel"),
            Format::Pascal => write!(f, "pascal"),
        }
    }
}

fn main() {
    let args = Args::parse();

    let verbs = ["searching", "talking", "walking", "eating", "running", "sleeping", "dancing", "reading", "writing", "swimming", "jumping", "singing", "cooking", "driving", "painting", "playing", "watching", "listening", "thinking", "building", "flying", "laughing", "crying", "hugging", "kissing", "fighting", "winning", "losing", "growing", "shrinking", "breathing", "climbing", "drawing", "exploring", "fishing", "gardening", "hiking", "inventing", "jogging", "knitting", "learning", "meditating", "navigating", "observing", "photographing", "questioning", "racing", "sailing", "teaching", "understanding", "vacationing", "whispering", "yawning", "zooming", "admiring", "baking", "chasing", "dreaming", "enjoying", "forgiving", "gathering", "hoping", "ignoring", "joking", "kicking", "loving", "moving", "noticing", "opening", "praying", "quitting", "resting", "smiling", "traveling", "unlocking", "viewing", "waving", "yearning", "acting", "bathing", "cleaning", "digging", "editing", "feeding", "grilling", "hunting", "investing", "juggling", "kissing", "lifting", "mixing", "napping", "organizing", "picking", "quarreling", "riding", "shopping", "typing", "unwinding", "visiting", "washing", "yodeling", "zapping", "camping", "drinking", "falling", "glowing", "healing", "itching", "joining", "keeping", "leaving", "marching", "nodding", "owning", "packing", "quoting", "rolling", "sneaking", "tasting", "voting", "waiting", "yelling", "zoning", "arranging", "borrowing", "escaping", "framing", "glancing", "holding", "itching", "judging", "kneeling", "launching", "measuring", "nesting", "offering", "pacing", "quizzing", "reaching", "serving", "taming", "wishing", "yelling", "zapping", "answering", "blocking", "calling", "drilling", "exchanging", "framing", "glimpsing", "hiding", "jamming", "kneeling", "licking", "marching", "naming", "offering", "peering", "quoting", "recording", "shining", "timing", "waiting", "yelling", "zapping", "aiming", "blinking", "chewing", "dropping", "folding", "glaring", "hopping", "itching", "joining", "knocking", "locking", "measuring", "needing", "opening", "pecking", "quivering", "rubbing", "sharing", "ticking", "unpacking", "vibrating", "whistling", "yawning", "arriving", "bouncing", "counting", "dripping", "floating", "gathering", "hurrying", "improving", "juggling", "kicking", "landing", "muttering", "nodding", "overhearing", "puffing", "quivering", "rattling", "spinning", "tumbling", "using", "vanishing", "whispering", "yelling", "zooming", "adapting", "blinking", "checking", "developing", "fading", "gazing", "humming", "imagining", "joining", "keeping", "looking", "meeting", "noticing", "overlooking", "peeling", "quivering", "reaching", "starting", "trembling", "understanding", "vibrating", "wondering", "yawning", "admiring", "blinking", "cooking", "drying", "folding", "glancing", "holding", "improving", "juggling", "knocking", "learning", "mimicking", "nodding", "opening", "pausing", "quivering", "reflecting", "staring", "thinking", "understanding", "vibrating", "wandering", "yawning", "zooming", "adjusting", "blinking", "changing", "delivering", "forming", "glancing", "hiding", "improving", "joining", "keeping", "listening", "muttering", "noting", "opening", "peering", "quitting", "reaching", "staring", "tasting", "unveiling", "viewing", "wondering", "yawning", "zooming", "alternating", "blinking", "diving", "floating", "glancing", "hunting", "improving", "joining", "keeping", "listening", "muttering", "nodding", "opening", "peering", "questioning", "reaching", "staring", "tasting", "unveiling", "viewing", "wondering", "yawning", "zooming", "anticipating", "blinking", "chasing", "drifting", "floating", "glancing", "hustling", "improving", "joining", "keeping", "listening", "muttering", "nodding", "opening", "peeking", "questioning", "reaching", "staring", "tasting", "unveiling", "viewing", "wondering", "yawning", "zooming", "approaching", "blinking", "chasing", "drifting", "floating", "glancing", "hustling", "improving", "joining", "keeping", "listening", "muttering", "nodding", "opening", "peeking", "questioning", "reaching", "staring", "tasting", "unveiling", "viewing", "wondering", "yawning", "zooming", "brushing", "cuddling", "frowning", "gazing", "helping", "ironing", "joking", "kneeling", "leaning", "moping", "nudging", "overtaking", "persuading", "quivering", "relaxing", "squeezing", "tugging", "uplifting", "venturing", "wandering", "yelling", "zooming", "baking", "crying", "dusting", "fishing", "gazing", "hopping", "itching", "joking", "kneeling", "licking", "moping", "nudging", "overhearing", "painting", "quivering", "rushing", "squeezing", "tugging", "uplifting", "wandering", "yelling", "zooming", "asking", "bumping", "calling", "dropping", "frowning", "gazing", "hitting", "itching", "jogging", "knocking", "leaping", "muttering", "nodding", "opening", "poking", "quivering", "rushing", "squeezing", "tapping", "uplifting", "winking", "yelling", "zooming", "applying", "baking", "climbing", "dressing", "folding", "gazing", "helping", "ironing", "joking", "knitting", "learning", "measuring", "noting", "opening", "packing", "quivering", "rinsing", "squeezing", "tidying", "unfolding", "wiping", "yawning", "zooming"];
    let nouns = ["laptop", "bathroom", "window", "banana", "phone", "kitchen", "book", "car", "tree", "mirror", "house", "door", "table", "chair", "computer", "garden", "river", "mountain", "city", "street", "beach", "ocean", "forest", "desert", "bridge", "tower", "castle", "ship", "plane", "train", "apple", "bicycle", "cloud", "doorway", "elephant", "flower", "guitar", "hat", "island", "jacket", "kite", "lamp", "moon", "notebook", "ocean", "piano", "quilt", "rose", "sun", "turtle", "umbrella", "violin", "whale", "xylophone", "yacht", "zebra", "airplane", "balloon", "camera", "diamond", "engine", "fountain", "globe", "helmet", "iceberg", "jungle", "key", "lantern", "museum", "nest", "orchard", "pyramid", "quarry", "rainbow", "statue", "temple", "universe", "volcano", "waterfall", "yogurt", "zoo", "anchor", "butterfly", "candle", "drum", "envelope", "feather", "glove", "honey", "ink", "jewel", "kettle", "leaf", "map", "needle", "owl", "pearl", "queen", "ring", "sword", "throne", "unicorn", "vase", "wagon", "yarn", "zodiac", "album", "brush", "cactus", "desk", "eraser", "folder", "gym", "houseplant", "island", "journal", "keyboard", "lunchbox", "magnet", "notebook", "organizer", "pencil", "ruler", "stapler", "textbook", "umbrella", "vase", "wallet", "yogurt", "zebra", "briefcase", "dresser", "eyebrow", "fingernail", "goggles", "handkerchief", "icecube", "jacket", "keychain", "lips", "mustache", "necklace", "overcoat", "poncho", "quilt", "ring", "scarf", "tiara", "uniform", "vneck", "watch", "yogurt", "zircon", "apartment", "bakery", "cafeteria", "diner", "elevator", "farmhouse", "garage", "hospital", "inn", "jail", "kitchen", "laundry", "motel", "nursery", "office", "pharmacy", "restaurant", "school", "tavern", "university", "villa", "warehouse", "youth", "zoo", "aquarium", "bridge", "canal", "dam", "fjord", "glacier", "harbor", "island", "jungle", "kayak", "lagoon", "marsh", "peninsula", "quarry", "river", "swamp", "tundra", "valley", "waterfall", "yard", "zenith", "asteroid", "comet", "dwarf", "eclipse", "galaxy", "horizon", "planet", "quasar", "satellite", "telescope", "universe", "void", "wormhole", "year", "zone", "blender", "dishwasher", "freezer", "grill", "heater", "iron", "juicer", "kettle", "lawnmower", "microwave", "oven", "pantry", "quiche", "refrigerator", "stove", "toaster", "vacuum", "washer", "yolk", "zebra", "accordion", "banjo", "cello", "drum", "euphonium", "flute", "guitar", "harp", "instrument", "keyboard", "lute", "mandolin", "oboe", "piccolo", "quartet", "recorder", "saxophone", "trombone", "ukulele", "violin", "whistle", "xylophone", "yodel", "zither", "apricot", "blueberry", "cherry", "date", "elderberry", "fig", "grape", "honeydew", "imbe", "jackfruit", "kiwi", "lemon", "mango", "nectarine", "orange", "papaya", "quince", "raspberry", "strawberry", "tangerine", "ugli", "vanilla", "watermelon", "xigua", "yuzu", "zucchini", "acorn", "bark", "cactus", "dandelion", "elm", "fir", "grass", "hibiscus", "ivy", "juniper", "kudzu", "lily", "moss", "nettle", "oak", "palm", "quince", "rose", "sunflower", "tulip", "umbrella", "violet", "wheat", "yucca", "zinnia", "abacus", "diary", "eraser", "folder", "graph", "highlighter", "ink", "journal", "knapsack", "lunchbox", "marker", "notebook", "organizer", "paper", "quill", "ruler", "stapler", "textbook", "unfold", "vocabulary", "worksheet", "yearbook", "zone", "ant", "butterfly", "caterpillar", "dragonfly", "earthworm", "firefly", "grasshopper", "hummingbird", "insect", "jellyfish", "katydid", "ladybug", "mosquito", "newt", "otter", "penguin", "quail", "rabbit", "squirrel", "turtle", "urchin", "vulture", "wombat", "yak", "zebra", "ant", "beetle", "cricket", "dragonfly", "earwig", "firefly", "gnat", "hornet", "isopod", "junebug", "katydid", "ladybug", "mantis", "nit", "otter", "pterodactyl", "quail", "roach", "scorpion", "tick", "urchin", "vulture", "wasp", "yak", "zebra", "alphabet", "book", "chapter", "diagram", "encyclopedia", "footnote", "glossary", "handbook", "index", "journal", "keynote", "legend", "manual", "novel", "outline", "preface", "quotation", "reference", "summary", "table", "volume", "workbook", "yearbook", "backpack", "blanket", "clock", "dog", "earrings", "fireplace", "garden", "hat", "igloo", "jungle", "kite", "lemon", "mango", "necklace", "orange", "parrot", "quilt", "rainbow", "sunflower", "tree", "umbrella", "van", "watermelon", "yellow", "zebra", "basket", "bedroom", "couch", "desk", "egg", "fork", "grape", "house", "ice", "jacket", "kitchen", "lamp", "mirror", "newspaper", "orange", "plate", "quiet", "radio", "sofa", "table", "umbrella", "vase", "window", "xylophone", "yard", "zebra", "backpack", "bookshelf", "calendar", "desk", "envelope", "folder", "globe", "hammer", "ink", "journal", "key", "letter", "mailbox", "notebook", "pencil", "ruler", "scissors", "tape", "umbrella", "wallet", "yellow", "zebra", "bag", "box", "card", "dress", "egg", "fan", "glass", "hat", "iron", "jacket", "key", "lamp", "mug", "nest", "orange", "pan", "quilt", "rug", "sock", "tent", "umbrella", "vest", "wallet", "yellow", "zoo", "basket", "bucket", "candle", "dish", "eraser", "fork", "glass", "hammer", "ink", "jar", "knife", "lock", "magnet", "napkin", "opener", "plate", "quilt", "rope", "spoon", "tape", "umbrella", "vase", "wallet", "yellow", "zucchini", "apron", "boots", "cap", "dress", "earmuffs", "fashion", "glasses", "hat", "jacket", "kimono", "leather", "mask", "necklace", "overalls", "poncho", "quilt", "ring", "scarf", "tiara", "uniform", "veil", "watch", "yarmulke", "zippers", "apple", "banana", "cherry", "date", "elderberry", "fig", "grape", "honeydew", "imbe", "jackfruit", "kiwi", "lemon", "mango", "nectarine", "orange", "papaya", "quince", "raspberry", "strawberry", "tangerine", "ugli", "vanilla", "watermelon", "xigua", "yuzu", "zucchini", "acorn", "bamboo", "cactus", "dahlia", "elm", "fern", "grass", "hibiscus", "iris", "juniper", "kelp", "lily", "moss", "nettle", "orchid", "palm", "quince", "rose", "sunflower", "tulip", "umbrella", "violet", "wheat", "yarrow", "zinnia"];

    let mut names: Vec<String> = Vec::new();

    for _ in 0..args.count {
        let verb = fastrand::choice(verbs).unwrap();
        let noun = fastrand::choice(nouns).unwrap();

        let output = match args.format {
            Format::Normal => format!("{} {}", verb, noun),
            Format::Kebab => format!("{}-{}", verb, noun),
            Format::Snake => format!("{}_{}", verb, noun),
            Format::Constant => format!("{}_{}", verb.to_uppercase(), noun.to_uppercase()),
            Format::Camel => format!("{}{}", verb, capitalize(noun)),
            Format::Pascal => format!("{}{}", capitalize(verb), capitalize(noun)),
        };

        names.push(output);
    }

    for (i, name) in names.iter().enumerate() {
        if args.count > 1 {
            print!("{}. ", i + 1);
        }
        println!("{}", name);
    }

    if args.copy && !names.is_empty() {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            let _ = clipboard.set_text(&names[0]);
            eprintln!("(Copied to clipboard)");
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}