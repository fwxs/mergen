use structopt::StructOpt;


#[tokio::main]
async fn main()  -> Result<(), mergen::error::Error> {

    mergen::commands::Args::from_args().handle(mergen::HttpReqwest::new()).await?;

    Ok(())
}
