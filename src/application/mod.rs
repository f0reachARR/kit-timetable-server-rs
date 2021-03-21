pub mod repositories {
    pub mod subject;
    pub mod user;

    pub use subject::*;
    pub use user::*;
}

pub mod usecases {
    mod subject;
    mod user;

    pub use subject::*;
    pub use user::*;
}

pub mod interactors {
    mod subject;
    #[cfg(test)]
    mod subject_test;

    pub use subject::SubjectInteractor;
}
