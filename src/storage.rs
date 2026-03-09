use crate::task::Task;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const TASKS_FILE_NAME: &str = "tasks.json";

pub fn load_tasks() -> Result<Vec<Task>> {
    if !Path::new(TASKS_FILE_NAME).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(TASKS_FILE_NAME).context("Ошибка чтения файла")?;

    let tasks: Vec<Task> = serde_json::from_str(&content).context("Ошибка парсинга JSON")?;

    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks).context("Ошибка сериализации задач")?;

    fs::write(TASKS_FILE_NAME, json).context("Ошибка записи файла")?;

    Ok(())
}
