use super::GqlUserDto;
use crate::domain::entities::user_td;

#[test]
fn from_user_entity_test() {
    let entity = user_td::get_user_entity_test_data();
    let actual = GqlUserDto::from_entity(user_td::get_user_entity_test_data());

    assert_eq!(actual.id.to_string(), entity.id);
}
