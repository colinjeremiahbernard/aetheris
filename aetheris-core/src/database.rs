use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        info!("Connecting to database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        info!("Running database migrations...");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;
        
        info!("Database connection established");
        
        Ok(Self { pool })
    }
    
    pub async fn health_check(&self) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>("SELECT true")
            .fetch_one(&self.pool)
            .await
    }
}