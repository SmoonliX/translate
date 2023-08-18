use std::collections::HashMap;
use std::io;

fn clear_screen() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}

fn create_translation_dicts() -> (HashMap<&'static str, &'static str>, HashMap<&'static str, &'static str>) {
    let mut dict_ru_to_en = HashMap::new();
    dict_ru_to_en.insert("привет", "hello");
    dict_ru_to_en.insert("мир", "world");
    dict_ru_to_en.insert("мама", "mother");
    dict_ru_to_en.insert("папа", "dad");
    dict_ru_to_en.insert("работа", "job");
    dict_ru_to_en.insert("практика", "practice");
    dict_ru_to_en.insert("первый", "first");
    dict_ru_to_en.insert("телефон", "telephone");
    dict_ru_to_en.insert("фонарик", "flashlight");
    dict_ru_to_en.insert("программирование", "programming");
    dict_ru_to_en.insert("логика", "logics");
    dict_ru_to_en.insert("письмо", "letter");

    let mut dict_en_to_ru = HashMap::new();
    dict_en_to_ru.insert("hello", "привет");
    dict_en_to_ru.insert("world", "мир");
    dict_en_to_ru.insert("mother", "мама");
    dict_en_to_ru.insert("dad", "папа");
    dict_en_to_ru.insert("job", "работа");
    dict_en_to_ru.insert("practice", "практика");
    dict_ru_to_en.insert("first", "первый");
    dict_ru_to_en.insert("telephone", "телефон");
    dict_ru_to_en.insert("flashlight", "фонарик");
    dict_ru_to_en.insert("programming", "программирование");
    dict_ru_to_en.insert("logics", "логика");
    dict_ru_to_en.insert("letter", "письмо");

    (dict_ru_to_en, dict_en_to_ru)
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

    clear_screen(); // Очистка экрана

    let mut input_text = String::new();
    println!("Введите текст для перевода:");
    io::stdin().read_line(&mut input_text).expect("Ошибка чтения строки");

    let mut choice = String::new();
    println!("Выберите направление перевода (1 - с русского на английский, 2 - с английского на русский):");
    io::stdin().read_line(&mut choice).expect("Ошибка чтения строки");

    let translated_text = match choice.trim() {
        "1" => translate_text(input_text.trim(), &dict_ru_to_en),
        "2" => translate_text(input_text.trim(), &dict_en_to_ru),
        _ => {
            println!("Некорректный выбор. Текст останется без изменений.");
            return;
        }
    };

    clear_screen(); // Очистка экрана
    println!("Перевод: {}", translated_text);
}
