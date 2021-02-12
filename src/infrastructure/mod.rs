pub mod gateways {
    mod subject;

    pub use subject::SubjectGateway;
}

pub mod presenters {
    pub mod subject;
}

pub mod frameworks {
    mod container;
    pub mod graphql;
    pub use container::UsecaseContainer;
}
