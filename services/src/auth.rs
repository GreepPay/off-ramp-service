pub mod auth {
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use models::auth::{
        models::user::user::{NewUser, UpdateUser, User},
        schema::users::{self, uuid},
    };

    /// Let create CRUD Operations

    // Get many users
    pub async fn get_many_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let mut connection = models::auth::establish_connection().await?;

        let results: Vec<User> = users::table
            .filter(users::id.gt(0))
            .select(User::as_select())
            .load(&mut connection)
            .await?;

        Ok(results)
    }

    // Create a new user
    pub async fn create_user(new_user: NewUser<'_>) -> Result<User, Box<dyn std::error::Error>> {
        let mut connection = models::auth::establish_connection().await?;

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut connection)
            .await?;

        let created_user = users::table
            .filter(uuid.eq(new_user.uuid))
            .first::<User>(&mut connection)
            .await?;

        Ok(created_user)
    }

    // Update an existing user
    pub async fn update_user(
        user_uuid: &str,
        user_update: UpdateUser<'_>,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let mut connection = models::auth::establish_connection().await?;

        diesel::update(users::table.filter(uuid.eq(user_uuid)))
            .set(&user_update)
            .execute(&mut connection)
            .await?;

        let updated_user = users::table
            .filter(uuid.eq(user_uuid))
            .first::<User>(&mut connection)
            .await?;

        Ok(updated_user)
    }

    // Delete a user
    pub async fn delete_user(user_uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut connection = models::auth::establish_connection().await?;

        diesel::delete(users::table.filter(uuid.eq(user_uuid)))
            .execute(&mut connection)
            .await?;

        Ok(())
    }
}
