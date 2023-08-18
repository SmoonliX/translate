use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn clear_screen() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}

fn load_dictionary(file_path: &str) -> HashMap<String, String> {
    let file = File::open(file_path).expect("Ошибка открытия файла");
    let reader = BufReader::new(file);
    
    let mut dict = HashMap::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<_> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                dict.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
            }
        }
    }
    
    dict
}

fn translate_text(input_text: &str, dict: &HashMap<String, String>) -> String {
    let mut result = String::new();
    for word in input_text.split_whitespace() {
        if let Some(translation) = dict.get(word) {
            result.push_str(translation);
        } else {
            panic!("Слово '{}' не найдено в словаре.", word);
        }
        result.push(' ');
    }
    result
}

fn main() {
    let dict = load_dictionary("dictionary.txt");

    clear_screen();

    let mut input_text = String::new();
    println!("Введите текст для перевода:");
    io::stdin().read_line(&mut input_text).expect("Ошибка чтения строки");

    let mut choice = String::new();
    println!("Выберите направление перевода (1 - с русского на английский, 2 - с английского на русский):");
    io::stdin().read_line(&mut choice).expect("Ошибка чтения строки");

    let translated_text = match choice.trim() {
        "1" => translate_text(input_text.trim(), &dict),
        "2" => {
            let reversed_dict: HashMap<String, String> = dict.iter().map(|(k, v)| (v.clone(), k.clone())).collect();
            translate_text(input_text.trim(), &reversed_dict)
        }
        _ => {
            println!("Некорректный выбор. Текст останется без изменений.");
            return;
        }
    };

    clear_screen();
    println!("Перевод: {}", translated_text);
}
