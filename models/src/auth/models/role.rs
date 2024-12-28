pub mod role {
    use diesel::{mysql::Mysql, prelude::*};
    use crate::auth::schema::roles;


    #[derive(Queryable, Selectable, Eq, PartialEq, Debug, Identifiable)]
    #[diesel(table_name = roles)]
    #[diesel(check_for_backend(Mysql))]
    pub struct Role {
        pub id: u64,
        pub uuid: String,
        pub name: String,
        pub created_at: Option<chrono::NaiveDateTime>,
        pub updated_at: Option<chrono::NaiveDateTime>,
        pub editable_name: Option<String>,
    }
}