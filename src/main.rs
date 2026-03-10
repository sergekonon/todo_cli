use todo_cli::{Cli, TaskList};

fn main() {
    // 1. Создаем список задач
    let mut list = TaskList::new();
    list.load_tasks();

    let cli = Cli::new();
    cli.run(&mut list);
}
