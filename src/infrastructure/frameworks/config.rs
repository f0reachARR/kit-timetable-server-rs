use envconfig::Envconfig;

#[derive(Debug, PartialEq, Eq, Envconfig)]
pub struct AppConfig {
    #[envconfig(from = "ES_URL")]
    pub elasticsearch_url: String,
    #[envconfig(from = "ES_SUBJECT_INDEX", default = "subjects")]
    pub elasticsearch_subject_index: String,
    #[envconfig(from = "JWT_SECRET", default = "secret")]
    pub jwt_secret: String,
    #[envconfig(from = "JWT_ISSUER", default = "issuer")]
    pub jwt_issuer: String,
}
