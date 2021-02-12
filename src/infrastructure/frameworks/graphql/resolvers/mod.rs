use self::subject::SubjectQueryRoot;
use async_graphql::MergedObject;

mod subject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(SubjectQueryRoot);

#[derive(MergedObject, Default)]
pub struct MutationRoot();
