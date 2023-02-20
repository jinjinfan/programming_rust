use std::io::{Write, Read};
use std::result;

use async_std::net;
use async_std::task;
use async_std::io::prelude::*;

async fn cheapo_request(host : &str, port : u16, path : &str) -> std::io::Result<String>{
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;
    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    Ok(response)
}

async fn cheapo_owing_request(host : String, port : u16, path : String) -> std::io::Result<String>{
    cheapo_request(&host, port, &path).await
}

pub async fn many_requests(requests : Vec<(String, u16, String)>) -> Vec<std::io::Result<String>>
{
    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn_local(cheapo_owing_request(host, port, path)));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

pub async fn many_requests_asynclocal(requests : Vec<(String, u16, String)>) -> Vec<std::io::Result<String>>
{
    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn_local(async move {
            cheapo_request(&host, port, &path).await
        }));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

async fn verify_password(password : &str, hash : &str, key : &str)
    -> Result<bool, argonautica::Error>
{
    // Make a copy, so the closure can be 'static
    let password = password.to_string();
    let hash = hash.to_string();
    let key = key.to_string();

    async_std::task::spawn_blocking(move || {
        argonautica::Verifier::default()
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(key)
            .verify()
    }).await
}

pub async fn many_requests_surf(urls : &[String])
    -> Vec<Result<String, surf::Exception>>
{
    let client = surf::Client::new();
    let mut handlers = vec![];
    for url in urls {
        let request = client.get(&url).recv_string();
        handlers.push(async_std::task::spawn(request));
    }

    let mut results = vec![];
    for handler in handlers {
        results.push(handler.await);
    }
    results
}

fn main() -> std::io::Result<()> {
    /// block_on
    let response = task::block_on(cheapo_request("example.com", 80, "/"))?;
    println!("{}", response);

    /// spawn_local
    let requests = vec![
        ("example.com".to_string(),         80,    "/".to_string()),
        ("www.red-bean.com".to_string(),    80,    "/".to_string()),
        ("en.wikipedia.org".to_string(),    80,    "/".to_string()),
    ];
    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }

    /// async block
    /*let serve_one = async {
        let listner = net::TcpListener::bind("localhost:8078").await?;
        let (mut socket, _addr) = listner.accept().await?;
    };*/
    let input = async_std::io::stdin();
    let future = async {
        let mut line = String::new();
        input.read_line(&mut line).await?;
        println!("Read line: {}", line);
        Ok::<(), std::io::Error>(())
    };

    let requests = &["http://example.com".to_string(),
                                  "http://www.red-bean.com".to_string(),
                                  "https://en.wikipedia.org/wiki/Canada".to_string(),
    ];
    let results = async_std::task::block_on(many_requests_surf(requests));
    for result in results {
        match result {
            Ok(response) => println!("*** {} \n", response),
            Err(err) => eprintln!("error: {} \n", err),
         }
    }
    Ok(())
}
