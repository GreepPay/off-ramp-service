pub mod notification {
    use diesel::{mysql::Mysql, prelude::*};
    use serde::{Deserialize, Serialize};

    #[derive(Queryable, Selectable, Eq, Serialize, Deserialize, PartialEq, Debug)]
    #[diesel(table_name = crate::notification::schema::notifications)]
    #[diesel(check_for_backend(Mysql))]
    pub struct Notification {
        pub id: u64,                                   // Unsigned<Bigint> maps to u64
        pub uuid: String,                              // Char maps to String
        pub title: String,                             // Text maps to String
        pub body: String,                              // Longtext maps to String
        pub type_: String,                             // Char with sql_name "type" maps to String
        pub model_type: Option<String>,                // Nullable<Char> maps to Option<String>
        pub user_id: Option<i32>,                      // Nullable<Integer> maps to Option<i32>
        pub model_type_id: Option<String>,             // Nullable<Char> maps to Option<String>
        pub extra_url: Option<String>,                 // Nullable<Text> maps to Option<String>
        pub created_at: Option<chrono::NaiveDateTime>, // Nullable<Timestamp> maps to Option<NaiveDateTime>
        pub updated_at: Option<chrono::NaiveDateTime>, // Nullable<Timestamp> maps to Option<NaiveDateTime>
        pub unread: bool,                              // Bool maps to bool
    }
}
