use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref EMAIL_REGEX:Regex = Regex::new(r"^[\d\w+\-.]{1,}@[\d\w]{1,}.([\d\w]{2,})(.[\d\w]{2,})?$").unwrap();
    static ref PASSWORD_REGEX:Regex = Regex::new(r"^[\d\w+.\-@ ]{8,20}$").unwrap();
}

pub struct ProcessedRegex {
    pub truth: bool,
    pub returns: String
}

pub async fn check_regex_email(email: &String) -> ProcessedRegex {
    let newE = email.trim().to_lowercase();
    ProcessedRegex {
        truth: EMAIL_REGEX.is_match(&newE),
        returns: newE.to_string()
    }
}
pub async fn check_regex_password(password: &String) ->  ProcessedRegex {
    let newP = password.trim().to_lowercase();
    ProcessedRegex {
        truth: PASSWORD_REGEX.is_match(&newP),
        returns: newP.to_string()
    }
}
pub async fn check_same_pw(pw1: &String, pw2:&String) -> ProcessedRegex {
    let newRe = pw2.trim().to_lowercase();
    ProcessedRegex {
        truth: pw1.eq(&newRe),
        returns: newRe.to_string()
    }
}