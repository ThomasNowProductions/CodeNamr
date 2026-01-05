use clap::Parser;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Output format", default_value = "normal")]
    format: String,
    #[arg(short = 'n', long, help = "Number of names to generate", default_value_t = 1)]
    count: u32,
    #[arg(short, long, help = "Copy first name to clipboard")]
    copy: bool,
    #[arg(short = 's', long, help = "Random seed for reproducible names")]
    seed: Option<u64>,
    #[arg(short = 'p', long, help = "Prefix to add to each name")]
    prefix: Option<String>,
    #[arg(short = 'u', long, help = "Suffix to add to each name")]
    suffix: Option<String>,
}

const VERBS: &[&str] = &[
    "searching", "talking", "walking", "eating", "running", "sleeping", "dancing", "reading",
    "writing", "swimming", "jumping", "singing", "cooking", "driving", "painting", "playing",
    "watching", "listening", "thinking", "building", "flying", "laughing", "crying", "hugging",
    "kissing", "fighting", "winning", "losing", "growing", "shrinking", "breathing", "climbing",
    "drawing", "exploring", "fishing", "gardening", "hiking", "inventing", "jogging", "knitting",
    "learning", "meditating", "navigating", "observing", "photographing", "questioning", "racing",
    "sailing", "teaching", "understanding", "vacationing", "whispering", "yawning", "zooming",
    "admiring", "baking", "chasing", "dreaming", "enjoying", "forgiving", "gathering", "hoping",
    "ignoring", "joking", "kicking", "loving", "moving", "noticing", "opening", "praying",
    "quitting", "resting", "smiling", "traveling", "unlocking", "viewing", "waving", "yearning",
    "acting", "bathing", "cleaning", "digging", "editing", "feeding", "grilling", "hunting",
    "investing", "juggling", "lifting", "mixing", "napping", "organizing", "picking", "quarreling",
    "riding", "shopping", "typing", "unwinding", "visiting", "washing", "yodeling", "zapping",
    "camping", "drinking", "falling", "glowing", "healing", "joining", "keeping", "leaving",
    "marching", "nodding", "owning", "packing", "quoting", "rolling", "sneaking", "tasting",
    "voting", "waiting", "yelling", "zoning", "arranging", "borrowing", "escaping", "framing",
    "glancing", "holding", "judging", "kneeling", "launching", "measuring", "nesting", "offering",
    "pacing", "quizzing", "reaching", "serving", "taming", "wishing", "answering", "blocking",
    "calling", "drilling", "exchanging", "glimpsing", "hiding", "jamming", "licking", "naming",
    "peering", "recording", "shining", "timing", "aiming", "blinking", "chewing", "dropping",
    "folding", "gazing", "helping", "igniting", "loading", "meeting",
];

const NOUNS: &[&str] = &[
    "laptop", "bathroom", "window", "banana", "phone", "kitchen", "book", "car", "tree", "mirror",
    "house", "door", "table", "chair", "computer", "garden", "river", "mountain", "city", "street",
    "beach", "ocean", "forest", "desert", "bridge", "tower", "castle", "ship", "plane", "train",
    "apple", "bicycle", "cloud", "doorway", "elephant", "flower", "guitar", "hat", "island",
    "jacket", "kite", "lamp", "moon", "notebook", "piano", "quilt", "rose", "sun", "turtle",
    "umbrella", "violin", "whale", "xylophone", "yacht", "zebra", "airplane", "balloon", "camera",
    "diamond", "engine", "fountain", "globe", "helmet", "iceberg", "jungle", "key", "lantern",
    "museum", "nest", "orchard", "pyramid", "quarry", "rainbow", "statue", "temple", "universe",
    "volcano", "waterfall", "yogurt", "zoo", "anchor", "butterfly", "candle", "drum", "envelope",
    "feather", "glove", "honey", "ink", "jewel", "kettle", "leaf", "map", "needle", "owl", "pearl",
    "queen", "ring", "sword", "throne", "unicorn", "vase", "wagon", "yarn", "zodiac", "album",
    "brush", "cactus", "desk", "eraser", "folder", "gym", "houseplant", "journal", "keyboard",
    "lunchbox", "magnet", "organizer", "pencil", "ruler", "stapler", "textbook", "wallet",
    "briefcase", "dresser", "eyebrow", "fingernail", "goggles", "handkerchief", "icecube",
    "keychain", "lips", "mustache", "necklace", "overcoat", "poncho", "scarf", "tiara", "uniform",
    "vneck", "watch", "zircon", "apartment", "bakery", "cafeteria", "diner", "elevator",
    "farmhouse", "garage", "hospital", "inn", "jail", "laundry", "motel", "nursery", "office",
    "pharmacy", "restaurant", "school", "tavern", "university", "villa", "warehouse", "youth",
    "aquarium", "canal", "dam", "fjord", "glacier", "harbor", "kayak", "lagoon", "marsh",
    "peninsula", "river", "swamp", "tundra", "valley", "yard", "zenith", "asteroid", "barnacle",
    "coral", "dune", "ecosystem", "fen", "gorge", "hill", "isthmus", "knoll", "lake", "meadow",
    "oasis", "plateau", "quagmire", "ridge", "savanna", "trail", "upland", "verge", "wasteland",
    "xeric", "zone",
];

fn main() {
    let args = Args::parse();

    if let Some(seed) = args.seed {
        fastrand::seed(seed);
    }

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let first_name = if args.count > 0 {
        generate_and_print(&args, &mut handle)
    } else {
        None
    };

    if args.copy {
        if let Some(name) = first_name {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if clipboard.set_text(&name).is_ok() {
                    eprintln!("\nCopied to clipboard");
                }
            }
        }
    }
}

fn generate_and_print(args: &Args, handle: &mut impl Write) -> Option<String> {
    let mut first_name = None;

    for i in 0..args.count {
        let verb = fastrand::choice(VERBS).copied().unwrap_or("running");
        let noun = fastrand::choice(NOUNS).copied().unwrap_or("app");

        let name = match args.format.to_lowercase().as_str() {
            "kebab" => format!("{}-{}-{}{}",
                args.prefix.as_deref().unwrap_or(""),
                verb,
                noun,
                args.suffix.as_deref().map(|s| format!("-{}", s)).unwrap_or_default()
            ).trim_matches('-').to_string(),
            "snake" => {
                let prefix_part = args.prefix.as_deref()
                    .map(|p| format!("{}_", p))
                    .unwrap_or_default();
                format!("{}{}_{}{}",
                    prefix_part,
                    verb,
                    noun,
                    args.suffix.as_deref().map(|s| format!("_{}", s)).unwrap_or_default()
                ).replace("__", "_")
            },
            "constant" => format!("{}{}_{}{}",
                args.prefix.as_ref().map(|p| p.to_uppercase() + "_").unwrap_or_default(),
                verb.to_uppercase(),
                noun.to_uppercase(),
                args.suffix.as_ref().map(|s| "_".to_string() + &s.to_uppercase()).unwrap_or_default(),
            ),
            "camel" => format!("{}{}{}{}",
                args.prefix.as_deref().unwrap_or(""),
                verb,
                capitalize(noun),
                capitalize(args.suffix.as_deref().unwrap_or("")),
            ),
            "pascal" => format!("{}{}{}{}",
                capitalize(args.prefix.as_deref().unwrap_or("")),
                capitalize(verb),
                capitalize(noun),
                capitalize(args.suffix.as_deref().unwrap_or("")),
            ),
            _ => {
                let mut parts = vec![];
                if let Some(ref prefix) = args.prefix {
                    parts.push(prefix.clone());
                }
                parts.push(verb.to_string());
                if let Some(ref suffix) = args.suffix {
                    parts.push(suffix.clone());
                }
                parts.push(noun.to_string());
                parts.join(" ").replace("  ", " ")
            }
        };

        if i == 0 {
            first_name = Some(name.clone());
        }

        writeln!(handle, "{}", name).ok();
    }

    first_name
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
