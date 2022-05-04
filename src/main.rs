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
        println!("{}", "\nenter guess:".cyan().bold().on_white());
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
        Word {
            data: "cat".to_string(),
            word_type: Types::Noun,
        },
        Word {
            data: "dog".to_string(),
            word_type: Types::Noun,
        },
        Word {
            data: "fish".to_string(),
            word_type: Types::Noun,
        },
        Word {
            data: "walk".to_string(),
            word_type: Types::Verb,
        },
        Word {
            data: "run".to_string(),
            word_type: Types::Verb,
        },
        Word {
            data: "jump".to_string(),
            word_type: Types::Verb,
        },
        Word {
            data: "happy".to_string(),
            word_type: Types::Adj,
        },
        Word {
            data: "sad".to_string(),
            word_type: Types::Adj,
        },
        Word {
            data: "angry".to_string(),
            word_type: Types::Adj,
        },
        Word {
            data: "scary".to_string(),
            word_type: Types::Adj,
        },
    ];
    let word = words[rand::thread_rng().gen_range(0, words.len() - 1)];
    let mut board = Board::new(Word {
        data: word,
        word_type: Types::Noun,
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
