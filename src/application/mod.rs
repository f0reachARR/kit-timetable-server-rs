pub mod repositories {
    mod subject;

    pub use subject::SubjectRepository;
}

pub mod usecases {
    mod subject;

    pub use subject::SubjectUsecase;
}

pub mod interactors {
    mod subject;

    pub use subject::SubjectInteractor;
}
