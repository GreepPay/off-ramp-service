use std::env;

use diesel::ConnectionError;
use diesel_async::{AsyncConnection, AsyncPgConnection};

use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::sql_types::BigInt;

use diesel_async::methods::LoadQuery;

/// Trait for adding pagination capabilities to queries
///
/// This trait provides the ability to paginate any query by implementing
/// a single method `paginate`.
pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

/// Default implementation of Paginate trait for any type
impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: DEFAULT_PER_PAGE,
            page,
            offset: (page - 1) * DEFAULT_PER_PAGE,
        }
    }
}

const DEFAULT_PER_PAGE: i64 = 10;

/// Structure representing a paginated query
///
/// # Fields
/// * `query` - The underlying query being paginated
/// * `page` - Current page number (1-based)
/// * `per_page` - Number of items per page
/// * `offset` - Calculated offset for SQL query
#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl<T> Paginated<T> {
    /// Sets the number of items per page
    ///
    /// # Arguments
    /// * `per_page` - Number of items to display per page
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated {
            per_page,
            offset: (self.page - 1) * per_page,
            ..self
        }
    }

    /// Executes the paginated query and returns the results along with pagination metadata
    ///
    /// # Returns
    /// A tuple containing:
    /// * Vector of query results
    /// * Total number of pages
    /// * Total number of records
    /// * Number of items per page
    pub async fn load_and_count_pages<'a, U>(
        self,
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<(Vec<U>, i64, i64, i64)>
    where
        T: 'a,
        U: Send + 'a,
        Self: LoadQuery<'a, AsyncPgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = diesel_async::RunQueryDsl::load::<(U, i64)>(self, conn).await?;
        let total = results.first().map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages, total, per_page))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<AsyncPgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}

/// Structure containing paginated results and metadata
///
/// # Fields
/// * `data` - Vector of paginated items
/// * `total_pages` - Total number of available pages
/// * `total_records` - Total number of records in the dataset
/// * `page` - Current page number
/// * `per_page` - Number of items per page
pub struct Pagination<T> {
    pub data: Vec<T>,
    pub total_pages: i64,
    pub total_records: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Establishes a connection to the PostgreSQL database using environment variables
///
/// # Returns
/// * `Ok(AsyncPgConnection)` - Successfully established database connection
/// * `Err(ConnectionError)` - Failed to establish connection
pub async fn establish_connection() -> Result<AsyncPgConnection, ConnectionError> {
    let db_url = std::env::var("DATABASE_URL").unwrap();

    dotenv::dotenv().ok();

    match AsyncPgConnection::establish(&env::var(&db_url).unwrap()).await {
        Ok(connection) => Ok(connection),
        Err(error) => {
            eprintln!("Error establishing connection: {}", error);
            Err(error)
        }
    }
}
