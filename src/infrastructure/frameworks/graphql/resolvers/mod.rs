use self::subject::SubjectQueryRoot;
use async_graphql::MergedObject;

mod subject;
mod user;

#[derive(MergedObject, Default)]
pub struct QueryRoot(SubjectQueryRoot, user::UserQueryRoot);

#[derive(MergedObject, Default)]
pub struct MutationRoot();
