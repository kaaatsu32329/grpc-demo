use hello_world::greeter_client::GreeterClient;
use hello_world::GreetRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

const SERVER_IP: &str = "http://[::1]:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect(SERVER_IP).await?;

    loop {
        let request = tonic::Request::new(GreetRequest {
            name: "Tonic".into(),
        });

        match client.say_hello(request).await {
            Ok(response) => {
                println!("------------------------------");
                println!("Show response");
                println!(
                    "  Data : {:?}",
                    response
                        .metadata()
                        .clone()
                        .into_headers()
                        .get("date")
                        .unwrap()
                );
                println!("  target name : {:?}", response.get_ref().target_name);
                println!("  reply : {:?}", response.get_ref().reply);

                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(_e) => {
                // println!("Error: {:?}", e);
                println!("Connection closed");
                break;
            }
        }
    }

    println!("Client is done");
    Ok(())
}
