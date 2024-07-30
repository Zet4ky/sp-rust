use reqwest::Error;
use rand::Rng;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut words = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..9 {

        let response: Result<Vec<String>, reqwest::Error> = reqwest::get("https://random-word-api.herokuapp.com/word?number=75")
            .await?
            .json()
            .await;

        match response {
            Ok(response_content) => {
                if let Some(response_words) = response_content.get(rng.gen_range(0..response_content.len())) {
                    let capitalized_word = format!("{}{}", response_words.chars().next().unwrap().to_uppercase(), &response_words[1..]);
                    let random_number = rng.gen_range(0..10);
                    words.push(format!("{}{}", capitalized_word, random_number));
                }
            },
            Err(e) => {
                eprintln!("Error making http request: {}", e);
                return Err(e);
            }
        }
    }

    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string();
    let username = uuid_string.split('-').next().unwrap();

    println!("Username: {}", username);
    println!("Password: {}", words.join("-"));
    
    Ok(())
}
