use std::collections::HashMap;

fn main() {
    // Создаем пустой hashmap с ключами типа &str и значениями типа u32
    let mut map: HashMap<u32, &str> = HashMap::new();
    
    // Вставляем элементы в hashmap
    map.insert(1, "One");
    map.insert(2, "Two");
    map.insert(3, "Three");
    println!("{map:#?}");
    
    // Получаем значение из hashmap по ключу
    let value = map.get(&2);
    match value {
        Some(val) => {
            println!("Значение для ключа 2: {}", val);
        },
        None => {
            println!(":/");
        }
    }
    
    // Удаляем элемент из hashmap по ключу
    map.remove(&1);
    println!("{map:#?}");
}