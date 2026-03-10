use crate::storage;
use crate::task::Task;
use crate::task_list::TaskList;
use anyhow::{Context, Result};
use std::io::{self, Write};

#[derive(Default)]
pub struct Cli {}

impl Cli {
    pub fn new() -> Self {
        Self::default()
    }

    /// Запускает интерактивный цикл
    pub fn run(&self, list: &mut TaskList) {
        loop {
            self.print_tasks(list);
            self.show_menu();

            let choice = match self.read_line() {
                Ok(val) => val,
                Err(err) => {
                    println!("Ошибка ввода: {}", err);
                    continue;
                }
            };

            match choice.as_str() {
                "1" => {
                    if let Err(e) = self.create_task(list) {
                        print!("Ошибка создания задачи: {}", e);
                    }
                }
                "2" => {
                    if let Err(e) = self.edit_task(list) {
                        print!("Ошибка редактирования задачи: {}", e);
                    }
                }
                "3" => {
                    if let Err(e) = self.delete_task(list) {
                        print!("Ошибка удаления задачи: {}", e);
                    }
                }
                "4" => {
                    if let Err(e) = storage::save_tasks(list.tasks()) {
                        print!("Ошибка сохранения задач: {}", e);
                    } else {
                        println!("Данные сохранены. До свидания!");
                        break;
                    }
                }
                _ => println!("Неверный ввод, попробуйте снова."),
            }
        }
    }

    fn print_tasks(&self, list: &TaskList) {
        println!("\n=== Список задач ===");
        let tasks = list.get_tasks();
        if tasks.is_empty() {
            println!("Список пуст.");
        } else {
            for task in tasks {
                let status = if task.done { "✓" } else { "○" };
                println!(
                    "{} [ID:{}] Порядок {}: {}",
                    status, task.id, task.order, task.title
                );
            }
        }
    }

    fn show_menu(&self) {
        println!("\n=== Меню ===");
        println!("1. Создать задачу");
        println!("2. Редактировать задачу");
        println!("3. Удалить задачу");
        println!("4. Сохранить и выйти");
        print!("Выберите действие (1-4): ");
        io::stdout().flush().unwrap();
    }

    fn create_task(&self, list: &mut TaskList) -> Result<()> {
        println!("\n--- Создание задачи ---");

        print!("Название: ");
        io::stdout().flush()?;
        let title = self.read_line()?;
        if title.is_empty() {
            println!("Название не может быть пустым.");
            return Ok(());
        }

        print!("Порядок (число): ");
        io::stdout().flush()?;
        let order: u32 = self.read_line()?.parse().context("Неверное число")?;

        let next_id = list.get_tasks().iter().map(|t| t.id).max().unwrap_or(0) + 1;

        list.add_task(Task {
            id: next_id,
            order,
            title,
            done: false,
        });

        println!("Задача добавлена!");
        Ok(())
    }

    fn edit_task(&self, list: &mut TaskList) -> Result<()> {
        println!("\n--- Редактирование задачи ---");

        print!("Введите ID задачи для редактирования: ");
        io::stdout().flush()?;
        let id: u32 = self.read_line()?.parse().context("Неверный ID")?;

        let task = match list.get_tasks().iter().find(|t| t.id == id) {
            Some(t) => t.clone(),
            None => {
                println!("Задача с ID {} не найдена.", id);
                return Ok(());
            }
        };

        let new_done = loop {
            println!(
                "Текущий статус: {}",
                if task.done {
                    "Выполнена"
                } else {
                    "Не выполнена"
                }
            );

            print!("Новый статус (0 - не выполнена, 1 - выполнена, Enter для пропуска): ");
            io::stdout().flush()?;

            match self.read_line()?.as_str() {
                "0" => break false,    // Возвращаем false из цикла
                "1" => break true,     // Возвращаем true из цикла
                "" => break task.done, // Возвращаем старое значение (пропуск)
                _ => println!("Неверный ввод, попробуйте снова."),
            }
        };

        println!("Текущее название: {}", task.title);
        print!("Новое название (Enter для пропуска): ");
        io::stdout().flush()?;
        let new_title = self.read_line()?;

        println!("Текущий порядок: {}", task.order);
        print!("Новый порядок (Enter для пропуска): ");
        io::stdout().flush()?;
        let new_order_str = self.read_line()?;

        list.remove_task(id);

        list.add_task(Task {
            id: task.id,
            order: if new_order_str.is_empty() {
                task.order
            } else {
                new_order_str.parse().context("Неверное число")?
            },
            title: if new_title.is_empty() {
                task.title
            } else {
                new_title
            },
            done: new_done,
        });

        println!("Задача обновлена!");
        Ok(())
    }

    fn delete_task(&self, list: &mut TaskList) -> Result<()> {
        println!("\n--- Удаление задачи ---");

        print!("Введите ID задачи для удаления: ");
        io::stdout().flush()?;
        let id: u32 = self.read_line()?.parse().context("Неверный ID")?;

        if list.remove_task(id).is_some() {
            println!("Задача удалена!");
        } else {
            println!("Задача с ID {} не найдена.", id);
        }
        Ok(())
    }

    fn read_line(&self) -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).context("Ошибка ввода")?;
        Ok(input.trim().to_string())
    }
}
