#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate strfmt;
extern crate toml;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use strfmt::strfmt;

#[derive(Debug, Deserialize)]
struct Config {
    emotes: HashMap<String, String>,
    mappings: HashMap<String, String>,
}

fn main() {
    let mut config_file = File::open("config.toml").unwrap();
    let mut config_file_str = String::new();
    config_file.read_to_string(&mut config_file_str).unwrap();
    let decoded: Config = toml::from_str(&config_file_str).unwrap();

    println!("#define IBUS_MACRO(z) {{SEND_STRING(SS_LCTRL(\"U\")); SEND_STRING(z\"\\n\")}}");

    for (name, emote) in decoded.emotes {
        let codepoint_map = emote.chars().map(|em| {
            let mut ch_string = em.escape_unicode().to_string();
            ch_string.retain(|c| c != 'u' && c != '\\' && c != '{' && c != '}');
            ch_string
        });

        print!("#define EMOTE_{}() {{", name);

        for cp in codepoint_map {
            print!("IBUS_MACRO(\"{}\");", cp);
        }

        print!("}}\n");
    }

    println!();

    let mut len_func_mappings: HashMap<usize, String> = HashMap::new();

    len_func_mappings.insert(1, "SEQ_ONE_KEY(KC_{key_1})".to_string());
    len_func_mappings.insert(2, "SEQ_TWO_KEY(KC_{key_1}, KC_{key_2})".to_string());
    len_func_mappings.insert(3, "SEQ_THREE_KEY(KC_{key_1}, KC_{key_2}, KC_{key_3})".to_string());
    len_func_mappings.insert(4, "SEQ_FOUR_KEY(KC_{key_1}, KC_{key_2}, KC_{key_3}, KC_{key_4})".to_string());
    len_func_mappings.insert(5, "SEQ_FIVE_KEY(KC_{key_1}, KC_{key_2}, KC_{key_3}, KC_{key_4}, KC_{key_5})".to_string());

    for (seq, name) in decoded.mappings {
        let macro_str = len_func_mappings.get(&(seq.len())).unwrap();

        let mut fmtargs = HashMap::new();

        for (it, key) in seq.to_uppercase().chars().enumerate() {
            fmtargs.insert(format!("key_{}", it+1), key);
        }

        println!("{} {{ EMOTE_{}() }}", strfmt(&macro_str, &fmtargs).unwrap(), name);
    }
}
