struct Vector<T> {
    data: Vec<T>,
}

impl<T: std::default::Default> Vector<T> {
    // ----------------
    // Создание вектора
    // ----------------
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    // ----------------
    // Изменение размера вектора
    // ----------------
    fn resize(&mut self, new_size: usize) where T: Clone {
        self.data.resize(new_size, Default::default());
    }

    // ----------------
    // Создание вектора с начальным размером
    // ----------------
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    
    // ----------------
    // Добавление элемента в конец вектора
    // ----------------
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }
    
    // ----------------
    // Удаление последнего элемента вектора и возврат его значения
    // ----------------
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    // ----------------
    // Удаление элемента с заданным индексом
    // ----------------
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    // ----------------
    // Получение элемента с заданным индексом
    // ----------------
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
    
}

fn main() {
    let mut vecs: Vector<i32> = Vector::new();
    
    vecs.push(1);
    vecs.push(2);
    vecs.push(3);
    
    println!("{:?}", vecs.pop());
    println!("{:?}", vecs.remove(1));
    println!("{:?}", vecs.get(1));

    vecs.resize(5);
}