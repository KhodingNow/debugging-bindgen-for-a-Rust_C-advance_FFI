use std::fs::File;
use std::io::Write;


fn main() {
   
   // let language = std::env::var("GREET_LANG").unwrap();    
    let language = std::env::var("GREET_LANG").unwrap_or_e          lse(|_| "en".to_string()); 

    let greeting = match language.as_ref() {
        "en" => "Hello",
        "es" => "!Holla",
        "el" => "Molo",
        "de" => "Satchmo",
        x => panic!("Unsuported language code {}", x),
    };

/*
    let rust_code = format!("fn greet() {{
        println!(\"{}\");  }}", greeting);
        
*/


    let rust_code = format!(
        r#"
pub fn greet() {{
    println!("{}");
}}
"#,
        greeting
    );

    

    let mut file = File::create("src/greet.rs").unwrap();
    file.write_all(rust_code.as_bytes()).unwrap();

    println!("cargo:warning=GREET_LANG={}", language);

    
}
