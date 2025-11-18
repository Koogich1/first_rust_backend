use diesel::prelude::*;

pub fn establish_connection() -> Result<PgConnection, Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let database_url = std::env::var("DATABASE_URL")?;
    let connection = PgConnection::establish(&database_url)?;
    Ok(connection)
}