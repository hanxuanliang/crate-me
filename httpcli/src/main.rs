use colored::Colorize;
use structopt::{StructOpt, clap::AppSettings::ColoredHelp};
use anyhow::{Result, anyhow};
use mime::Mime;
use reqwest::{Client, Url, Response, header};
use std::{collections::HashMap, str::FromStr};

#[derive(StructOpt, Debug)]
#[structopt(name = "httpcli")]
#[structopt(about = "A simple HTTP client")]
#[structopt(author = "hanxuanliang")]
#[structopt(setting = ColoredHelp)]
struct Opts {
    #[structopt(subcommand)]
    sub: SubCmd,
}

#[derive(StructOpt, Debug)]
enum SubCmd {
    Get(Get),
    Post(Post),
}

#[derive(StructOpt, Debug)]
struct Get {
    #[structopt(parse(try_from_str = parse_url))]
    url: String,
}

#[derive(StructOpt, Debug)]
struct Post {
    /// http url
    #[structopt(parse(try_from_str = parse_url))]
    url: String,
    /// http body 
    #[structopt(parse(try_from_str = parse_kvpair))]
    body: Vec<KvPair>,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("=");
        let err = || anyhow!(format!("Fail to parse {}", s));
        Ok(Self {
            k: (iter.next().ok_or_else(err)?).to_string(),
            v: (iter.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kvpair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::from_args();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust HTTPcli".parse()?);
    let client = Client::builder()
        .default_headers(headers).build()?;
    let res = match opts.sub {
        SubCmd::Get(ref args) => http_get(client, args).await?, 
        SubCmd::Post(ref args) => http_post(client, args).await?
    };

    Ok(res)
}

async fn http_get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{}", resp.text().await?);
    Ok(())
}

async fn http_post(client: Client, args: &Post) -> Result<()> {
    let mut form = HashMap::new();
    for kv in args.body.iter() {
        form.insert(&kv.k, &kv.v);
    }
    let resp = client.post(&args.url).json(&form).send().await?;

    Ok(print_resp(resp).await?)
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);  
    Ok(())
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), 
        resp.status());
    println!("{}\n", status.blue());
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!("\n")
}

fn print_body(m: Option<Mime>, body: &str) {
    match m {
        Some(m) if m == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        },
        _ => println!("{}", body),
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}
