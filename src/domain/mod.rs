pub mod entities {
    pub mod subject;
    #[cfg(test)]
    pub mod subject_td;

    pub use subject::SubjectEntity;
}
