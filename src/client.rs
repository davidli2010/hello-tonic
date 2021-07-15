use hello::greeter_client::GreeterClient;
use hello::HelloRequest;
use tonic::transport::Channel;

mod hello;

impl GreeterClient<Channel> {
    pub async fn connect(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = Channel::builder(url.parse()?);
        let channel = endpoint.connect().await?;
        Ok(GreeterClient::new(channel))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:5000").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
