use clap::Parser;
use rand::seq::SliceRandom;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Output format", default_value = "normal")]
    format: String,
}

fn main() {
    let args = Args::parse();

    let verbs = ["searching", "talking", "walking", "eating", "running", "sleeping", "dancing", "reading", "writing", "swimming", "jumping", "singing", "cooking", "driving", "painting", "playing", "watching", "listening", "thinking", "building", "flying", "laughing", "crying", "hugging", "kissing", "fighting", "winning", "losing", "growing", "shrinking", "breathing", "climbing", "drawing", "exploring", "fishing", "gardening", "hiking", "inventing", "jogging", "knitting", "learning", "meditating", "navigating", "observing", "photographing", "questioning", "racing", "sailing", "teaching", "understanding", "vacationing", "whispering", "x-raying", "yawning", "zooming", "admiring", "baking", "chasing", "dreaming", "enjoying", "forgiving", "gathering", "hoping", "ignoring", "joking", "kicking", "loving", "moving", "noticing", "opening", "praying", "quitting", "resting", "smiling", "traveling", "unlocking", "viewing", "waving", "xylophoning", "yearning", "zipping", "acting", "bathing", "cleaning", "digging", "editing", "feeding", "grilling", "hunting", "investing", "juggling", "kissing", "lifting", "mixing", "napping", "organizing", "picking", "quarreling", "riding", "shopping", "typing", "unwinding", "visiting", "washing", "xeroing", "yodeling", "zapping"];
    let nouns = ["laptop", "bathroom", "window", "banana", "phone", "kitchen", "book", "car", "tree", "mirror", "house", "door", "table", "chair", "computer", "garden", "river", "mountain", "city", "street", "beach", "ocean", "forest", "desert", "bridge", "tower", "castle", "ship", "plane", "train", "apple", "bicycle", "cloud", "doorway", "elephant", "flower", "guitar", "hat", "island", "jacket", "kite", "lamp", "moon", "notebook", "ocean", "piano", "quilt", "rose", "sun", "turtle", "umbrella", "violin", "whale", "xylophone", "yacht", "zebra", "airplane", "balloon", "camera", "diamond", "engine", "fountain", "globe", "helmet", "iceberg", "jungle", "key", "lantern", "museum", "nest", "orchard", "pyramid", "quarry", "rainbow", "statue", "temple", "universe", "volcano", "waterfall", "xerox", "yogurt", "zoo", "anchor", "butterfly", "candle", "drum", "envelope", "feather", "glove", "honey", "ink", "jewel", "kettle", "leaf", "map", "needle", "owl", "pearl", "queen", "ring", "sword", "throne", "unicorn", "vase", "wagon", "x-ray", "yarn", "zodiac"];

    let mut rng = rand::thread_rng();
    let verb = verbs.choose(&mut rng).unwrap();
    let noun = nouns.choose(&mut rng).unwrap();

    let output = match args.format.as_str() {
        "normal" => format!("{} {}", verb, noun),
        "dash" => format!("{}-{}", verb, noun),
        "upper" => format!("{}_{}", verb.to_uppercase(), noun.to_uppercase()),
        "lower" => format!("{}_{}", verb, noun),
        "camel" => format!("{}{}", verb, capitalize(noun)),
        "pascal" => format!("{}{}", capitalize(verb), capitalize(noun)),
        _ => format!("{} {}", verb, noun),
    };

    println!("{}", output);
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}