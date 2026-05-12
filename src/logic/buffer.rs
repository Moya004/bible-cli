use array_deque::ArrayDeque as Deque;
use rusqlite::Error;

use crate::business::repositories::BufferRepository;

pub struct Buffer {
    history: Deque<String>,
    history_pointer: usize,
}

//TODO: Agregar load buffer history
impl Buffer {
    pub fn new() -> Self {
        Self {
            history: Deque::new(10_000),
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
        if self.history.len() > 0 && self.history_pointer < self.history.len() - 1 {
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

    pub fn load_history<T: BufferRepository>(&mut self, repo: &T) -> Result<(), Error> {
        match repo.load_history(10_000 as u16 - self.history.len() as u16) {
            Ok(history) => {
                self.history.extend(history);
            }
            Err(error) => return Err(error),
        }
        Ok(())
    }

    pub fn save_history<T: BufferRepository>(&self, repo: &T) -> Result<(), Error> {
        match repo.save_history(self.history.clone()) {
            Ok(nothing) => Ok(nothing),
            Err(error) => Err(error),
        }
    }
}
