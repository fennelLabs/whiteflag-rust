pub enum MessageBodyType {
    //GENERIC,
    AUTHENTICATION(Authentication),
    /* CRYPTO,
    TEXT,
    RESOURCE,
    TEST,
    SIGNAL,
    REQUEST, */
}

pub struct Authentication {
    /// e.g. 1
    verification_method: usize,
    /// e.g. https://organisation.int/whiteflag
    verification_data: String,
}

impl Default for Authentication {
    fn default() -> Self {
        Self {
            verification_method: 1,
            verification_data: "https://organisation.int/whiteflag".to_string(),
        }
    }
}
