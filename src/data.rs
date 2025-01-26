use chrono::NaiveDate;
use serde::{Deserialize, Serialize, Serializer, de};

pub trait DataSource: de::DeserializeOwned + Serialize {
    const NAME: &str;
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct About {
    pub name: String,
    pub image: String,
    #[serde(serialize_with = "markdown_to_html")]
    pub description: String,
    pub profiles: Vec<Profile>,
    pub work_experience: Vec<WorkExperience>,
    pub projects: Vec<Project>,
    pub education: Vec<Education>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub name: String,
    pub text: String,
    pub link: String,
    pub svg: String,
    #[serde(default)]
    exclude_from_resume: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkExperience {
    organization: String,
    link: String,
    title: String,
    skills: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    location: String,
    #[serde(serialize_with = "markdown_to_html")]
    description: String,
    #[serde(default)]
    exclude_from_resume: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    name: String,
    link: String,
    skills: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    #[serde(serialize_with = "markdown_to_html")]
    description: String,
    #[serde(default)]
    exclude_from_resume: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Education {
    school: String,
    link: String,
    degree: String,
    tagline: Option<String>,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    location: String,
    #[serde(default)]
    exclude_from_resume: bool,
}

impl DataSource for About {
    const NAME: &str = "about";
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub pages: Vec<Page>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    pub name: String,
    pub link: String,
}

impl DataSource for Config {
    const NAME: &str = "config";
}

fn markdown_to_html<S: Serializer>(s: &str, ser: S) -> Result<S::Ok, S::Error> {
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, pulldown_cmark::Parser::new(s));
    ser.serialize_str(&html)
}
