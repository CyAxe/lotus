use lotus::threader::Threader;

#[tokio::main]
async fn main() {
    let threader = Threader::init(30,30);
    threader.start(vec!["http://nokia.com","http://php.net","http://google.com"],|url| {
        println!("SOMETIMES I THINK {}",url);
    }).await;
}
