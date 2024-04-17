use tokio::time::{self, Duration};

pub async fn user_runtime_scheduler() {
    async fn periodic_task() {
        // Ваша функция, которая будет вызываться каждый определенный интервал времени
        println!("Выполняется периодическая задача в user_runtime");
    }

    
    
    let interval = Duration::from_secs(2);
  periodic_task().await;

  time::sleep(interval).await;

}