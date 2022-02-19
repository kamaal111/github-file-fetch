use clap::Parser;
use std::path::Path;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    user: String,

    #[clap(short, long)]
    repo: String,

    #[clap(short, long)]
    branch: String,

    #[clap(short, long)]
    filepath: String,

    #[clap(short, long, default_value_t = String::from("./"))]
    output: String,
}

fn main() {
    let args = Args::parse();
    let filepath = args.filepath;
    let url = make_url(&args.user, &args.repo, &args.branch, &filepath);

    let response = make_request(url);

    let filename = filepath.split("/").last().unwrap();
    let output_file_path = Path::new(&args.output).join(filename);
    std::fs::write(output_file_path, response).expect("unable to write file");
}

fn make_request(url: String) -> String {
    let response = reqwest::blocking::get(url).unwrap();
    let body = response.text().unwrap();
    body
}

fn make_url(user: &str, repository: &str, branch: &str, filepath: &str) -> String {
    let mut url = Url::parse("https://raw.githubusercontent.com").unwrap();

    let strings_to_join = [user, repository, branch, filepath];
    for string_to_join in strings_to_join {
        let mut string_to_join = string_to_join.to_string();
        let last_character = string_to_join.chars().last().unwrap();
        if last_character != '/' {
            string_to_join.push_str("/");
        }

        url = url
            .join(&string_to_join)
            .expect(&format!("could not join {} in to url", string_to_join));
    }

    let mut url_string = url.to_string();
    url_string.pop();
    url_string
}
