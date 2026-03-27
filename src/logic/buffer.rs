use array_deque::StackArrayDeque as SDeque;

pub struct Buffer {
    history: SDeque<String, 10_000>,
    history_pointer: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            history: SDeque::new(),
            history_pointer: 0,
        }
    }

    pub fn save_on_history(&mut self, content: String) {
        self.history.push_back(content.clone());

        if self.history_pointer <= self.history.len() {
            self.history_pointer = self.history.len();
        }
    }

    pub fn load_prev_entry(&mut self, content: &mut String) {
        if self.history_pointer > 0 {
            self.history_pointer -= 1;
            *content = self.history[self.history_pointer].clone();
        }
    }

    pub fn load_next_entry(&mut self, content: &mut String) {
        if self.history_pointer < self.history.len() - 1 {
            self.history_pointer += 1;
            *content = self.history[self.history_pointer].clone();
        }
    }
    pub fn read_line(&mut self, input: String) -> String {
        self.save_on_history(input.clone());

        let input_to_return: String = match input.trim().parse() {
            Ok(text) => text,
            Err(_) => return input,
        };

        input_to_return
    }
}
