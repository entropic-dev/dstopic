use std::fs;
use std::io;
use std::sync::Arc;

use fluent::{FluentBundle, FluentResource};
use fluent_locale::{negotiate_languages, NegotiationStrategy};

const L10N_RESOURCES: &[&str] = &["hello.ftl"];
const LOCALES_PATH: &str = "./locales";

fn read_file(path: &str) -> Result<String, io::Error> {
    let s = fs::read_to_string(path)?;
    Ok(s)
}

pub fn get_resources(requested: &str) -> Result<Vec<Arc<FluentResource>>, io::Error> {
    let mut resources: Vec<Arc<FluentResource>> = vec![];
    let locales = get_app_locales(&[requested]).expect("Failed to retrieve available locales");

    for path in L10N_RESOURCES {
        let full_path = format!(
            "{LOCALES_PATH}/{locale}/{path}",
            LOCALES_PATH = LOCALES_PATH,
            locale = locales[0],
            path = path
        );
        let source = read_file(&full_path).expect("Failed to read file.");
        let resource = FluentResource::try_new(source).expect("Could not parse an FTL string.");
        resources.push(Arc::new(resource));
    }

    Ok(resources)
}

pub fn get_resource_bundle(requested: &str) -> Result<FluentBundle, io::Error> {
    let locales = get_app_locales(&[requested]).expect("Failed to retrieve available locales");
    let mut bundle = FluentBundle::new(&locales);

    let resources = get_resources(requested)?;

    for res in resources {
        bundle
            .add_resource(res)
            .expect("Failed to add resource to the bundle");
    }

   Ok(bundle)
}

pub fn get_locales() -> Result<Vec<String>, io::Error> {
    let mut locales = vec![];
    let res_dir = fs::read_dir(LOCALES_PATH)?;

    for entry in res_dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name) = name.to_str() {
                        locales.push(String::from(name));
                    }
                }
            }
        }
    }

    Ok(locales)
}

fn get_app_locales(requested: &[&str]) -> Result<Vec<String>, io::Error> {
    let available = get_locales()?;
    let resolved_locales = negotiate_languages(
        requested,
        &available,
        Some("en-US"),
        &NegotiationStrategy::Filtering,
    );
    return Ok(resolved_locales
        .into_iter()
        .map(|s| String::from(s))
        .collect::<Vec<String>>());
}

#[cfg(test)]
mod test {
    use crate::localization::get_resource_bundle;

    #[test]
    pub fn test_get_resource_bundle() {
        let es_mx_locale = "es-MX";
        let es_mx_bundle = get_resource_bundle(es_mx_locale).unwrap();
        let (es_mx_value, _) = es_mx_bundle
                    .format("hello-world", None)
                    .expect("Failed to format a message");
        assert_eq!(es_mx_value, String::from("Hola mundo"));

        let en_us_locale = "en-US";
        let en_us_bundle = get_resource_bundle(en_us_locale).unwrap();
        let (en_us_value, _) = en_us_bundle
                    .format("hello-world", None)
                    .expect("Failed to format a message");
        assert_eq!(en_us_value, String::from("Hello world!"));
    }
}
