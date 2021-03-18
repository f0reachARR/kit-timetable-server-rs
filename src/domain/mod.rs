pub mod entities {
    pub mod subject;
    #[cfg(test)]
    pub mod subject_td;

    pub use subject::{SubjectEntity, SubjectSearchTermsEntity};

    pub mod user;
    #[cfg(test)]
    pub mod user_td;

    pub use user::UserEntity;
}
