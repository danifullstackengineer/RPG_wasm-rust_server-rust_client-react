#[cfg(test)]
mod tests {
    
    use super::super::utils::ProcessedRegex;

    use super::super::ajax::verify_and_register;
    use super::super::utils::{check_regex_email, check_regex_password, check_same_pw};
    use tokio;

    #[tokio::test(flavor = "multi_thread")]
    async fn check_email_regex_test() {
        let bad_emails = ["b@", "bssdds1\\", "kejifeje12322@co", "ksksdhsuhds@s.c", "email with space between @ my email .com", "mybademail@with@three.do.ma.ins"];
        let good_emails = ["babadany2999@gmail.co", " hello@google.co ", "mygoodemail@pepe.co.uk"];
        for email in bad_emails {
            let bad_processed_regex = ProcessedRegex {
                truth: false,
                returns: email.to_string()
            };
            let result = check_regex_email(&email.to_string()).await;
            assert_eq!(
                bad_processed_regex.truth, result.truth
            );
            assert_eq!(
                bad_processed_regex.returns,
                result.returns
            )
        }
        for email in good_emails {
            let good_processed_regex =  ProcessedRegex {
                truth: true,
                returns: email.trim().to_lowercase().to_string()
            };
            let result = check_regex_email(&email.to_string()).await;
            assert_eq!(
                good_processed_regex.truth, result.truth
            );
            assert_eq!(
                good_processed_regex.returns, result.returns
            );
        }
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn check_password_regex_test() {
        let bad_passwords = ["small", "tooofreakinghugeitdoesntevenfit", "invalidchars/=", "\\escapingchar"];
        let good_passwords = ["ihavegoodlength", r"igood1W+.-@", "  space in "];

        for pw in bad_passwords {
            let bad_processed_req = ProcessedRegex {
                truth: false,
                returns: pw.to_string()
            };
            let result = check_regex_password(&pw.to_string()).await;
            assert_eq!(
                bad_processed_req.truth, result.truth
            );
            assert_eq!(
                bad_processed_req.returns, result.returns
            );
        }
        for pw in good_passwords {
            let good_processed_req = ProcessedRegex {
                truth: true,
                returns: pw.trim().to_lowercase().to_string()
            };
            let result = check_regex_password(&pw.to_string()).await;
            assert_eq!(
                good_processed_req.truth, result.truth
            );
            assert_eq!(
                good_processed_req.returns, result.returns
            );
        }
    }
    #[test]
    fn check_same_password_regex() {
        let pw1 = "mysecretpassword";
        let pw2 = "notmysecretpassword";

        assert_ne!(pw1, pw2);

        let pw2_same = "mysecretpassword";

        assert_eq!(pw1, pw2_same);
    }

}