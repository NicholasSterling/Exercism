const CALM: &str = "Calm down, I know what I'm doing!";
const WHOA: &str = "Whoa, chill out!";
const SURE: &str = "Sure.";
const WHATEVER: &str = "Whatever.";
const FINE: &str = "Fine. Be that way!";

pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    let any = |f: fn(char) -> bool| {
        msg.chars().any(f)
    };
    let is_yelled = any(char::is_uppercase) && !any(char::is_lowercase);
    if msg.is_empty() {
        FINE
    } else {
        let is_question = msg.ends_with("?");
        if is_yelled {
            if is_question { CALM } else { WHOA }
        } else {
            if is_question { SURE } else { WHATEVER }
        }
    }
}
