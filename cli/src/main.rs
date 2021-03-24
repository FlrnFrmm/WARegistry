mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = app::create();
    
    let matches = app.get_matches();

    println!("---\n");
    println!("{:?}", matches);
    println!("\n---");

    app::process_matches(&matches).await?;
    
    Ok(())
}
