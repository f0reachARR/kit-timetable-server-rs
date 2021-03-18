use chrono::TimeZone;

use super::user::*;

pub fn get_user_entity_test_data() -> UserEntity {
    UserEntity {
        id: "testing:testing".into(),
        created_at: chrono::Utc.ymd(2014, 7, 8).and_hms_micro(9, 10, 11, 12_000),
    }
}
