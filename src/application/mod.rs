pub mod repositories {
    pub mod subject;
    pub mod subject_mock;

    pub use subject::*;
}

pub mod usecases {
    mod subject;

    pub use subject::*;
}

pub mod interactors {
    mod subject;

    pub use subject::SubjectInteractor;
}
