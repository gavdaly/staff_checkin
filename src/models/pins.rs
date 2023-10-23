use cfg_if::cfg_if;

use uuid::Uuid;

pub struct Pin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub number: u32,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use chrono::NaiveDateTime;

    struct SqlPin {
        number: u32,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        user_id: Uuid,
        id: Uuid,
    }

    impl SqlPin {
        async fn create_pin(id: Uuid) -> Self {
            // destroy old pin
            // create new pin
            // return self
            //
            Self {
                number: 1111,
                created_at: NaiveDateTime::default(),
                updated_at: NaiveDateTime::default(),
                user_id: Uuid::new_v4(),
                id: Uuid::new_v4(),
            }
        }
    }
}
}
