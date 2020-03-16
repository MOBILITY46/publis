mod bundle;
mod s3;
mod utils;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Publis", about = "Publish files to S3")]
struct Opt {
    #[structopt(short, long)]
    bucket: String,

    #[structopt(short, long, help = "Your bundle root directory")]
    root: Option<String>,

    #[structopt(
        short,
        long,
        takes_value = false,
        help = "Add policy to serve static content"
    )]
    add_policy: bool,

    #[structopt(
        long,
        takes_value = false,
        help = "Dry run. List files that would be uploaded"
    )]
    dry: bool,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    if opt.add_policy {
        let result = crate::bundle::add_bucket_policy(&opt.bucket).await;

        match result {
            Ok(policy) => println!("Policy: {:?}", policy),
            Err(err) => eprintln!("{}", err),
        }
    }

    if let Some(root) = opt.root {
        let result = crate::bundle::upload_all(&root, &opt.bucket, opt.dry).await;

        match result {
            Ok(()) => {
                if opt.dry == false {
                    println!("Bundle successfully uploaded.")
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}
