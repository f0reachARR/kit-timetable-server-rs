use std::sync::Arc;

use crate::application::usecases::SubjectUsecase;

pub struct UsecaseContainer {
    pub subject_usecase: Arc<dyn SubjectUsecase>,
}
