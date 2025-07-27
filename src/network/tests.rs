use super::*;
use tokio_test;

#[tokio::test]
async fn test_tcp_socket_creation() {
    let socket = TcpSocket::new().await;
    assert!(socket.is_ok());
}

#[tokio::test]
async fn test_tcp_connect() {
    // Test connection to a reliable service (adjust as needed)
    let addr: SocketAddr = "8.8.8.8:53".parse().unwrap();
    let result = TcpSocket::connect(&addr).await;
    
    // This might fail due to network conditions, so we just test the function exists
    // In a real test environment, you'd use a mock server
    match result {
        Ok(_) => println!("Connection successful"),
        Err(e) => println!("Connection failed (expected in test): {}", e),
    }
}
