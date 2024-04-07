use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    root_dir: PathBuf,
    port: String,
}

impl Server {
    pub fn new(root_dir: &str, port: &u16) -> Self {
        Server {
            root_dir: PathBuf::from(root_dir),
            port: port.to_string(),
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .unwrap();

        println!("🚀 Server is running on port {}", self.port); 
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let root_dir = self.root_dir.clone(); // Clone the PathBuf here

            tokio::spawn(async move {
                if let Err(e) = Server::handle_connection(root_dir, stream).await {
                    eprintln!("Failed to handle connection: {}", e);
                }
            });
        }
    }

    async fn handle_connection(root_dir: PathBuf, mut stream: TcpStream) -> io::Result<()> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await?;
        let buffer_string = String::from_utf8_lossy(&buffer[..]);
        let request_line = buffer_string.lines().next().unwrap_or_default();
        let request_path = Server::parse_request(&request_line);

        let mut file_path = root_dir.join(request_path.strip_prefix("/").unwrap_or(""));
        if !file_path.is_file() {
            file_path = root_dir.join("index.html");
        }

        let mut file = File::open(&file_path).await?;

        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;

        let mime_type = Server::get_mime_type(&file_path);
        let response_header = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            mime_type,
            contents.len()
        );

        stream.write_all(response_header.as_bytes()).await?;
        stream.write_all(&contents).await?;
        stream.flush().await?;

        Ok(())
    }

    fn get_mime_type(path: &Path) -> &str {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("json") => "application/json",
            _ => "application/octet-stream", // default binary data MIME type
        }
    }

    fn parse_request(request_line: &str) -> String {
        request_line.split(' ').nth(1).unwrap_or("/").to_string()
    }
}
