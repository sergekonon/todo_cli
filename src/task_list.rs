use crate::storage;
use crate::task::Task;

#[derive(Default)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        match storage::load_tasks() {
            Ok(tasks) => TaskList { tasks },
            Err(_) => TaskList::default(),
        }
    }

    pub fn load_tasks(&mut self) {
        if let Ok(tasks) = storage::load_tasks() {
            self.tasks = tasks;
        }
    }

    pub fn add_task(&mut self, task: Task) -> usize {
        let idx = self.tasks.partition_point(|t| t.order <= task.order);
        self.tasks.insert(idx, task);
        idx
    }

    pub fn remove_task(&mut self, id: u32) -> Option<Task> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            Some(self.tasks.remove(pos))
        } else {
            None
        }
    }

    pub fn get_tasks(&self) -> &[Task] {
        &self.tasks
    }

    // Метод для получения доступа к вектору (нужен для сохранения)
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    // Метод для замены содержимого (нужен для загрузки)
    pub fn set_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }
}
