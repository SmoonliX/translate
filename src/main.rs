use std::collections::HashMap;
use std::io;

// Простые словари для демонстрации перевода
fn create_translation_dicts() -> (HashMap<&'static str, &'static str>, HashMap<&'static str, &'static str>) {
    let mut dict_ru_to_en = HashMap::new();
    dict_ru_to_en.insert("привет", "hello");
    dict_ru_to_en.insert("мир", "world");
    dict_ru_to_en.insert("мама", "mother");
    dict_ru_to_en.insert("папа", "dad");
    dict_ru_to_en.insert("работа", "job");
    dict_ru_to_en.insert("практика", "practice");

    let mut dict_en_to_ru = HashMap::new();
    dict_en_to_ru.insert("hello", "привет");
    dict_en_to_ru.insert("world", "мир");
    dict_en_to_ru.insert("mother", "мама");
    dict_en_to_ru.insert("dad", "папа");
    dict_en_to_ru.insert("job", "работа");
    dict_en_to_ru.insert("practice", "практика");
    // При желании можно добавить больше слов

    (dict_ru_to_en, dict_en_to_ru) // Возвращение слов с выбором перевода
}

fn translate_text(input_text: &str, dict: &HashMap<&str, &str>) -> String {
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
    let (dict_ru_to_en, dict_en_to_ru) = create_translation_dicts();

    println!("Введите текст для перевода:");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Ошибка чтения строки");

    println!("Выберите направление перевода (1 - с русского на английский, 2 - с английского на русский):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Ошибка чтения строки");

    let translated_text = match choice.trim() {
        "1" => translate_text(input_text.trim(), &dict_ru_to_en),
        "2" => translate_text(input_text.trim(), &dict_en_to_ru),
        _ => {
            println!("Некорректный выбор. Текст останется без изменений.");
            return; 
        }
    };
    println!("Перевод: {}", translated_text);
}
