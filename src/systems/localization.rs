use std::collections::HashMap;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub struct I18nManager {
    pub current_lang: String,
    translations: HashMap<String, HashMap<String, String>>,
}

impl I18nManager {
    pub fn new(initial_lang: &str) -> Self {
        let mut manager = Self {
            current_lang: initial_lang.to_string(),
            translations: HashMap::new(),
        };
        
        manager.load_xml_from_str("en", include_str!("../../assets/translations/en.xml"));
        manager.load_xml_from_str("id", include_str!("../../assets/translations/id.xml"));
        
        manager
    }

    pub fn set_language(&mut self, lang: &str) {
        self.current_lang = lang.to_string();
    }

    pub fn load_xml_from_str(&mut self, lang: &str, xml_data: &str) {
        let mut reader = Reader::from_str(xml_data);
        reader.config_mut().trim_text(true);
        
        let mut map = HashMap::new();
        let mut current_key = String::new();
        let mut in_string = false;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"string" {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"key" {
                                    current_key = String::from_utf8_lossy(&attr.value).into_owned();
                                    in_string = true;
                                }
                            }
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_string {
                        map.insert(current_key.clone(), e.unescape().unwrap_or_default().into_owned());
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"string" {
                        in_string = false;
                        current_key.clear();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!("Error parsing XML translation for {}: {:?}", lang, e);
                    break;
                }
                _ => (),
            }
        }
        self.translations.insert(lang.to_string(), map);
    }

    pub fn t(&self, key: &str) -> String {
        if let Some(lang_map) = self.translations.get(&self.current_lang) {
            if let Some(val) = lang_map.get(key) {
                return val.clone();
            }
        }
        if self.current_lang != "en" {
            if let Some(en_map) = self.translations.get("en") {
                if let Some(val) = en_map.get(key) {
                    return val.clone();
                }
            }
        }
        format!("[{}]", key)
    }

    pub fn t_var(&self, key: &str, vars: &[(&str, &str)]) -> String {
        let mut text = self.t(key);
        for (k, v) in vars {
            text = text.replace(&format!("{{{}}}", k), v);
        }
        text
    }
}
