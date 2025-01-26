use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialOrd, Ord, PartialEq, Hash)]
pub struct EmailAddress(String);

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid email address")]
pub struct EmailAddressError(String);

impl EmailAddress {
    pub fn new(raw_email: &str) -> Result<Self, EmailAddressError> {
        if email_regex(raw_email) {
            let lower = raw_email.to_lowercase();
            Ok(Self(lower))
        } else {
            Err(EmailAddressError(raw_email.into()))
        }
    }
}

fn email_regex(raw_email: &str) -> bool {
    true
}

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl EmailAddress {
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for EmailAddress {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn demonstrate() {
    let raw = "EMAIL@ADDRESS.COM";
    let email = EmailAddress::new(raw).unwrap();
    // NOTE: works becuase of Deref trait
    takes_a_str(&email);
    // email.deref() // can work as well

    let lower = email.to_lowercase();
    assert_eq!(lower, "email@address.com");

    assert_eq!(*raw, *email);
}

fn takes_a_str(_: &str) {}

// impl PartialEq for EmailAddress {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.eq_ignore_ascii_case(&other.0)
//     }
// }
//
// impl std::hash::Hash for EmailAddress {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.to_ascii_lowercase().hash(state);
//     }
// }

impl std::borrow::Borrow<str> for EmailAddress {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_no_no_no_no() {
        use std::collections::HashMap;
        let mut login_attempts: HashMap<EmailAddress, i32> = HashMap::new();

        let raw_uppercase = "EMAIL@TEST.COM";
        let raw_lowercase = "email@test.com";

        let email = EmailAddress::new(raw_uppercase).unwrap();

        login_attempts.insert(email, 2);

        let got_attempts = login_attempts.get(raw_lowercase);
        assert_eq!(got_attempts, Some(&2));
    }
}
