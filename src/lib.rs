pub mod cli;
pub mod storage;
pub mod task;
pub mod task_list;

pub use cli::Cli;
pub use task::Task;
pub use task_list::TaskList;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sorting() {
        // Тесты логики сортировки
    }
}
