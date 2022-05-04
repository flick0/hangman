use colored::*;
use rand::Rng;
use std::io;

#[derive(Debug)]
enum Types {
    Noun,
    Verb,
    Adj,
}
impl Types {
    fn to_string(&self) -> &str {
        match self {
            Types::Noun => "noun",
            Types::Verb => "verb",
            Types::Adj => "adj",
        }
    }
}

struct Word {
    data: String,
    word_type: Types,
}

struct Board {
    word: Word,
    map: Vec<(String, String)>,
    guessed: Vec<char>,
}
impl Board {
    fn new(word: Word) -> Board {
        let mut map: Vec<(String, String)> = Vec::new();
        for i in 0..word.data.len() {
            map.push((
                word.data.chars().nth(i).unwrap().to_string(),
                "_".to_string(),
            ));
        }
        Board {
            word: word,
            map: map,
            guessed: Vec::new(),
        }
    }
    fn draw(&self) {
        println!("");
        for ch in &self.map {
            if ch.1 != "_" {
                print!("{}", ch.1.green());
            } else {
                print!("{}", ch.1);
            }
        }
        println!("\n{}", self.word.word_type.to_string().yellow().italic());
        // for ch in &self.map {
        //     print!("{}", ch.0);
        // }
    }
    fn guess(&mut self) -> bool {
        let mut guess = String::new();
        println!("{}", "\nenter guess:".blue().bold().on_white());
        io::stdin()
            .read_line(&mut guess)
            .expect("\nFailed to read line");
        let guess = guess.trim();
        if guess.len() != 1 {
            println!("{}", "\nenter only one character".red().bold());
            return false;
        }
        for ch in &self.guessed {
            if ch == &guess.chars().nth(0).unwrap() {
                println!("{}", "\nalready guessed".yellow().bold());
                return false;
            }
        }
        let mut to_return: bool = false;
        for i in 0..self.word.data.len() {
            if self.word.data.chars().nth(i).unwrap() == guess.chars().nth(0).unwrap() {
                self.map[i].1 = guess.to_string();
                self.guessed.push(guess.chars().nth(0).unwrap());
                to_return = true;
            }
        }
        if !to_return {
            println!("{}", "\nno! , try again".red().bold());
        }
        return to_return;
    }
    fn check_win(&self) -> bool {
        let mut count: u32 = self.word.data.len() as u32;
        for ch in &self.map {
            if ch.1 == ch.0 {
                count -= 1
            }
        }
        if count == 0 {
            return true;
        }
        return false;
    }
}

fn main() {
    let words = vec![
        ("banana", "noun"),
        ("apple", "noun"),
        ("orange", "noun"),
        ("grape", "noun"),
        ("pear", "noun"),
        ("watermelon", "noun"),
        ("strawberry", "noun"),
        ("kiwi", "noun"),
        ("peach", "noun"),
        ("pineapple", "noun"),
        ("coconut", "noun"),
        ("run", "verb"),
        ("walk", "verb"),
        ("fly", "verb"),
        ("swim", "verb"),
        ("sing", "verb"),
        ("dance", "verb"),
        ("laugh", "verb"),
        ("cry", "verb"),
        ("eat", "verb"),
        ("drink", "verb"),
        ("sleep", "verb"),
        ("play", "verb"),
        ("watch", "verb"),
        ("read", "verb"),
        ("write", "verb"),
        ("red", "adj"),
        ("blue", "adj"),
        ("green", "adj"),
        ("yellow", "adj"),
        ("orange", "adj"),
        ("purple", "adj"),
        ("pink", "adj"),
        ("brown", "adj"),
        ("black", "adj"),
        ("white", "adj"),
        ("gray", "adj"),
        ("pale", "adj"),
        ("dark", "adj"),
        ("light", "adj"),
        ("warm", "adj"),
        ("cold", "adj"),
        ("hot", "adj"),
        ("dry", "adj"),
        ("wet", "adj"),
        ("sweet", "adj"),
    ];
    let len_words = words.len();
    let &word = words
        .get(rand::thread_rng().gen_range(0..len_words))
        .unwrap();
    let mut board = Board::new(Word {
        data: word.0.to_string(),
        word_type: match word.1.to_lowercase().as_str() {
            "noun" => Types::Noun,
            "verb" => Types::Verb,
            "adj" => Types::Adj,
            _ => Types::Noun,
        },
    });
    loop {
        board.draw();
        board.guess();
        if board.check_win() {
            println!("{}", "\nYOU WIN!".green().bold());
            board.draw();
            break;
        }
    }
}
