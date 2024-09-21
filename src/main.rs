use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

mod bootloader;
use bootloader::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	// 监听地址
	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

	// 绑定监听地址
	let listener = TcpListener::bind(addr).await?;

	// 开始阻塞等待连接
	loop {
		let (stream, _) = listener.accept().await?;

		let io = TokioIo::new(stream);

		tokio::task::spawn(async move {
			if let Err(err) = http1::Builder::new()
				.serve_connection(io, service_fn(bootloader))
				.await
			{
				println!("Error serving connection: {:?}", err);
			}
		});
	}
}
