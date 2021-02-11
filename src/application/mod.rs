pub mod repositories {
    mod subject;

    pub use subject::SubjectRepository;
}

pub mod usecases {
    mod subject;

    pub use subject::SubjectUsecase;
}
