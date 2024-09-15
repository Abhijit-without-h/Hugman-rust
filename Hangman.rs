use std::io;
use std::io::Write;
use rand::seq::SliceRandom;

fn main() {
    let words = vec!["rust", "programming", "computer", "algorithm", "development",
    "abruptly",
    "absurd",
    "abyss",
    "affix",
    "askew",
    "avenue",
    "awkward",
    "axiom",
    "azure",
    "bagpipes",
    "bandwagon",
    "banjo",
    "bayou",
    "beekeeper",
    "bikini",
    "blitz",
    "blizzard",
    "boggle",
    "bookworm",
    "boxcar",
    "boxful",
    "buckaroo",
    "buffalo",
    "buffoon",
    "buxom",
    "buzzard",
    "buzzing",
    "buzzwords",
    "caliph",
    "cobweb",
    "cockiness",
    "croquet",
    "crypt",
    "curacao",
    "cycle",
    "daiquiri",
    "dirndl",
    "disavow",
    "dizzying",
    "duplex",
    "dwarves",
    "embezzle",
    "equip",
    "espionage",
    "euouae",
    "exodus",
    "faking",
    "fishhook",
    "fixable",
    "fjord",
    "flapjack",
    "flopping",
    "fluffiness",
    "flyby",
    "foxglove",
    "frazzled",
    "frizzled",
    "fuchsia",
    "funny",
    "gabby",
    "galaxy",
    "galvanize",
    "gazebo",
    "giaour",
    "gizmo",
    "glowworm",
    "glyph",
    "gnarly",
    "gnostic",
    "gossip",
    "grogginess",
    "haiku",
    "haphazard",
    "hyphen",
    "iatrogenic",
    "icebox",
    "injury",
    "ivory",
    "ivy",
    "jackpot",
    "jaundice",
    "jawbreaker",
    "jaywalk",
    "jazziest",
    "jazzy",
    "jelly",
    "jigsaw",
    "jinx",
    "jiujitsu",
    "jockey",
    "jogging",
    "joking",
    "jovial",
    "joyful",
    "juicy",
    "jukebox",
    "jumbo",
    "kayak",
    "kazoo",
    "keyhole",
    "khaki",
    "kilobyte",
    "kiosk",
    "kitsch",
    "kiwifruit",
    "klutz",
    "knapsack",
    "larynx",
    "lengths",
    "lucky",
    "luxury",
    "lymph",
    "marquis",
    "matrix",
    "megahertz",
    "microwave",
    "mnemonic",
    "mystify",
    "naphtha",
    "nightclub",
    "nowadays",
    "numbskull",
    "nymph",
    "onyx",
    "ovary",
    "oxidize",
    "oxygen",
    "pajama",
    "peekaboo",
    "phlegm",
    "pixel",
    "pizazz",
    "pneumonia",
    "polka",
    "pshaw",
    "psyche",
    "puppy",
    "puzzling",
    "quartz",
    "queue",
    "quips",
    "quixotic",
    "quiz",
    "quizzes",
    "quorum",
    "razzmatazz",
    "rhubarb",
    "rhythm",
    "rickshaw",
    "schnapps",
    "scratch",
    "shiv",
    "snazzy",
    "sphinx",
    "spritz",
    "squawk",
    "staff",
    "strength",
    "strengths",
    "stretch",
    "stronghold",
    "stymied",
    "subway",
    "swivel",
    "syndrome",
    "thriftless",
    "thumbscrew",
    "topaz",
    "transcript",
    "transgress",
    "transplant",
    "triphthong",
    "twelfth",
    "twelfths",
    "unknown",
    "unworthy",
    "unzip",
    "uptown",
    "vaporize",
    "vixen",
    "vodka",
    "voodoo",
    "vortex",
    "voyeurism",
    "walkway",
    "waltz",
    "wave",
    "wavy",
    "waxy",
    "wellspring",
    "wheezy",
    "whiskey",
    "whizzing",
    "whomever",
    "wimpy",
    "witchcraft",
    "wizard",
    "woozy",
    "wristwatch",
    "wyvern",
    "xylophone",
    "yachtsman",
    "yippee",
    "yoked",
    "youthful",
    "yummy",
    "zephyr",
    "zigzag",
    "zigzagging",
    "zilch",
    "zipper",
    "zodiac",
    "zombie"
];
    let word = words.choose(&mut rand::thread_rng()).unwrap();
    let mut guessed_letters = Vec::new();
    let mut attempts_left = 6;

    println!("Welcome to Hangman!");
    println!("The word has {} letters.", word.len());

    while attempts_left > 0 {
        print_word_progress(word, &guessed_letters);
        println!("Attempts left: {}", attempts_left);
        print!("Enter a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().chars().next().unwrap_or(' ');

        if !guess.is_alphabetic() {
            println!("Please enter a valid letter.");
            continue;
        }

        if guessed_letters.contains(&guess) {
            println!("You've already guessed that letter.");
            continue;
        }

        guessed_letters.push(guess);

        if !word.contains(guess) {
            attempts_left -= 1;
            println!("Incorrect guess!");
        } else {
            println!("Correct guess!");
        }

        if word_guessed(word, &guessed_letters) {
            println!("Congratulations! You've guessed the word: {}", word);
            return;
        }
    }

    println!("Game over! The word was: {}", word);
}

fn print_word_progress(word: &str, guessed_letters: &Vec<char>) {
    for c in word.chars() {
        if guessed_letters.contains(&c) {
            print!("{} ", c);
        } else {
            print!("_ ");
        }
    }
    println!();
}

fn word_guessed(word: &str, guessed_letters: &Vec<char>) -> bool {
    word.chars().all(|c| guessed_letters.contains(&c))
}