#![cfg(all(feature = "requests", feature = "retry", not(target_arch = "wasm32")))]

use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

struct RecordedRequest {
    target: String,
    authorization: String,
    body: Vec<u8>,
}

fn serve(statuses: Vec<&'static str>) -> (String, JoinHandle<Vec<RecordedRequest>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        let mut requests = Vec::new();
        for status in statuses {
            let deadline = Instant::now() + Duration::from_secs(10);
            let stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(error)
                        if error.kind() == std::io::ErrorKind::WouldBlock
                            && Instant::now() < deadline =>
                    {
                        thread::sleep(Duration::from_millis(1));
                    }
                    Err(error) => panic!("waiting for test request: {error}"),
                }
            };
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            stream
                .set_write_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut stream = BufReader::new(stream);
            let mut line = String::new();
            stream.read_line(&mut line).unwrap();
            let target = line.split_whitespace().nth(1).unwrap().to_owned();
            let mut authorization = String::new();
            let mut content_length = 0;
            loop {
                line.clear();
                assert_ne!(stream.read_line(&mut line).unwrap(), 0);
                if line == "\r\n" {
                    break;
                }
                let (name, value) = line.split_once(':').unwrap();
                if name.eq_ignore_ascii_case("authorization") {
                    authorization = value.trim().to_owned();
                } else if name.eq_ignore_ascii_case("content-length") {
                    content_length = value.trim().parse().unwrap();
                }
            }
            let mut body = vec![0; content_length];
            stream.read_exact(&mut body).unwrap();
            requests.push(RecordedRequest {
                target,
                authorization,
                body,
            });

            let response = r#"{"results":[]}"#;
            write!(
                stream.get_mut(),
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nContent-Type: \
                 application/json\r\nConnection: close\r\n\r\n{response}",
                response.len()
            )
            .unwrap();
        }
        requests
    });
    (base_url, server)
}

fn client(base_url: &str) -> crate::Client {
    let builder = || {
        reqwest::Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(2))
    };
    let mut client = crate::Client::new_from_reqwest("test-token", builder(), builder());
    client.set_base_url(base_url);
    client
}

#[tokio::test]
async fn generated_queries_retry_transient_errors() {
    let (base_url, server) = serve(vec!["503 Service Unavailable", "200 OK"]);
    let response = client(&base_url)
        .users()
        .get_settings_v_3_get_page(Some("cursor +&".to_owned()), Some(2))
        .await
        .unwrap();
    assert!(response.results.is_empty());

    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 2);
    for request in requests {
        let url = url::Url::parse(&format!("{base_url}{}", request.target)).unwrap();
        assert_eq!(url.path(), "/settings/v3/users/");
        assert_eq!(
            url.query_pairs().into_owned().collect::<BTreeMap<_, _>>(),
            BTreeMap::from([
                ("after".to_owned(), "cursor +&".to_owned()),
                ("limit".to_owned(), "2".to_owned())
            ])
        );
        assert_eq!(request.authorization, "Bearer test-token");
    }
}

#[tokio::test]
async fn streaming_multipart_is_sent_once_without_retry() {
    let (base_url, server) = serve(vec!["503 Service Unavailable"]);
    let client = client(&base_url);
    let request = client
        .client
        .post(format!("{base_url}/upload"))
        .multipart(reqwest::multipart::Form::new().text("payload", "streamed-body"))
        .build()
        .unwrap();
    assert!(request.try_clone().is_none());

    let response = client.client.execute(request).await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].target, "/upload");
    assert!(String::from_utf8_lossy(&requests[0].body).contains("streamed-body"));
}
