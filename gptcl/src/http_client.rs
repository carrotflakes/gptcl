pub trait HttpClient: std::fmt::Debug {
    fn post<'a>(
        &'a self,
        url: &'a str,
        api_key: &'a str,
        body: String,
    ) -> std::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>,
                > + Send
                + 'a,
        >,
    >;
}
