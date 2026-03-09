use crate::task::Task;

#[derive(Default)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_task(&mut self, task: Task) {
        // Вставка с сохранением сортировки по order
        let idx = self.tasks.partition_point(|t| t.order <= task.order);
        self.tasks.insert(idx, task);
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
