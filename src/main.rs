use url::Url;

fn main() {
    let user = "kamaal111";
    let repository = "AdventOfCode";
    let branch = "main";
    let filepath = "2021/day1/input.txt";
    let url = make_url(user, repository, branch, filepath);
    println!("{}", url);
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
            .expect("could not join path in url");
    }

    url.to_string()
}
