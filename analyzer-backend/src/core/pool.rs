use crate::analyzer::Analyzer;
use deadpool::unmanaged::Pool;
use std::sync::Arc;
async fn create_pool(
    engine_path: &str,
) -> Result<Arc<Pool<Analyzer>>, Box<dyn std::error::Error + Send + Sync>> {
    let pool = Pool::new(4); // Говорим, что максимум будет 4 объекта

    // 2. Создаём ваши 4 движка и кладём их в пул
    for _ in 0..4 {
        let engine = Analyzer::new(engine_path)?; // Ваша инициализация
        pool.add(engine).await?;
    }

    // 3. Оборачиваем в Arc для шаринга между NATS и Axum
    let shared_pool = Arc::new(pool);
    Ok(shared_pool)
}
