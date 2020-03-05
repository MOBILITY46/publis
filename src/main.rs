mod bundle;
mod s3;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Publis", about = "Publish files to S3")]
struct Opt {
    #[structopt(short, long)]
    root: String,

   #[structopt(short, long)]
    bucket: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let result = crate::bundle::upload_all(&opt.root, &opt.bucket).await;

    match result {
        Ok(()) => println!("Bundle successfully uploaded."),
        Err(err) => eprintln!("{}", err),
    }
}
