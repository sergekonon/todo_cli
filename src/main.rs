use anyhow::Result;
use todo_cli::{Cli, TaskList, storage};

fn main() -> Result<()> {
    // 1. Создаем список задач
    let mut list = TaskList::new();

    // 2. Загружаем данные из файла (Инфраструктура -> Бизнес)
    let loaded_tasks = storage::load_tasks()?;
    list.set_tasks(loaded_tasks);

    let cli = Cli::new();
    cli.run(&mut list)?;

    // 4. Сохраняем данные в файл (Бизнес -> Инфраструктура)
    // storage::save_tasks(list.tasks())?;

    Ok(())
}

// Функция выводит в консоль список задач, предлагает создать новую задачу, редактировать задачу, удалять задачу
