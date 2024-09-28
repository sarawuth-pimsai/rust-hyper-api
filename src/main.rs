async fn test() -> String {
    "hello".to_string()
}
#[derive(Clone)]
struct Service;
impl hyper::service::Service<hyper::Request<hyper::body::Incoming>> for Service {
    type Response = hyper::Response<http_body_util::Full<hyper::body::Bytes>>;

    type Error = hyper::Error;

    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn call(&self, req: hyper::Request<hyper::body::Incoming>) -> Self::Future {
        match (req.method(), req.uri().path()) {
            (&hyper::Method::GET, "/") => Box::pin(async {
                let word = test().await;
                Ok(hyper::Response::builder()
                    .body(http_body_util::Full::new(hyper::body::Bytes::from(word)))
                    .unwrap())
            }),
            _ => Box::pin(async {
                let word = test().await;
                Ok(hyper::Response::builder()
                    .body(http_body_util::Full::new(hyper::body::Bytes::from(word)))
                    .unwrap())
            }),
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let service = Service;
    loop {
        let (steam, _) = listener.accept().await?;
        let io = hyper_util::rt::TokioIo::new(steam);
        let instance = service.clone();
        tokio::task::spawn(async move {
            if let Err(e) = hyper::server::conn::http1::Builder::new()
                .serve_connection(io, instance)
                .await
            {
                eprint!("Error: {:?}", e);
            }
        });
    }
}
