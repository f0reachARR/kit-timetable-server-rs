pub mod gateways {
    mod subject;
    mod user;

    pub use subject::SubjectGateway;
    pub use user::UserGateway;
}

pub mod controllers {
    pub mod subject;
}

pub mod presenters {
    pub mod subject;
}

pub mod frameworks {
    pub mod config;
    mod container;
    pub mod graphql;
    pub use container::UsecaseContainer;
}
