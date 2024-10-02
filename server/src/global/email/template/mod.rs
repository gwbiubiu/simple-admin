use lazy_static::lazy_static;
use tera::Tera;
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("server/src/global/email/template/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}
