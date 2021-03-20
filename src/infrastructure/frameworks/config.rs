use envconfig::Envconfig;

#[derive(Debug, PartialEq, Eq, Envconfig)]
pub struct AppConfig {
    #[envconfig(from = "ES_URL")]
    pub elasticsearch_url: String,
    #[envconfig(from = "ES_SUBJECT_INDEX", default = "subjects")]
    pub elasticsearch_subject_index: String,
}
