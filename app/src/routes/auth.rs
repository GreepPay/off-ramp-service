pub mod auth {
    use controllers::{
        api::api::{failure, success, ApiResponse},
        auth::form::form::{CreateUserForm, UpdateUserForm},
    };
    use models::auth::models::user::user::User;
    use rocket::{
        delete, form::Form, get, http::Status, post, put, response::status, serde::json::Json,
    };

    #[get("/")]
    pub async fn get_users(
    ) -> Result<status::Custom<Json<ApiResponse<Vec<User>>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let many_users = controllers::auth::get_many_users()
            .await
            .map_err(|_| failure("Failed to get many user", Status::InternalServerError))?;

        Ok(success("User fetched successfully", many_users, Status::Ok))
    }

    #[post("/", data = "<form>")]
    pub async fn add_user<'r>(
        form: Form<CreateUserForm<'r>>,
    ) -> Result<status::Custom<Json<ApiResponse<User>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let new_user = controllers::auth::create_new_user(form)
            .await
            .map_err(|_| failure("Failed to create new user", Status::InternalServerError))?;

        Ok(status::Custom(
            Status::Ok,
            ApiResponse::success("User created successfully".to_string(), new_user),
        ))
    }

    #[put("/<user_uuid>", data = "<form>")]
    pub async fn update_user<'r>(
        user_uuid: &'r str,
        form: Form<UpdateUserForm<'r>>,
    ) -> Result<status::Custom<Json<ApiResponse<User>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let updated_user = controllers::auth::update_user(&user_uuid, form)
            .await
            .map_err(|_| failure("Failed to update user", Status::InternalServerError))?;

        Ok(status::Custom(
            Status::Ok,
            ApiResponse::success("User updated successfully".to_string(), updated_user),
        ))
    }

    #[delete("/<user_uuid>")]
    pub async fn delete_user<'r>(
        user_uuid: &'r str,
    ) -> Result<status::Custom<Json<ApiResponse<()>>>, status::Custom<Json<ApiResponse<()>>>> {
        controllers::auth::delete_user(&user_uuid)
            .await
            .map_err(|_| failure("Failed to delete user", Status::InternalServerError))?;

        Ok(status::Custom(
            Status::Ok,
            ApiResponse::success("User deleted successfully".to_string(), ()),
        ))
    }
}
