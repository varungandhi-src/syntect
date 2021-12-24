//! Prints highlighted HTML for a file to stdout.
//! Basically just wraps a body around `highlighted_html_for_file`
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_string;

fn main() {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please pass in a file to highlight");
        return;
    }

    use std::time::*;
    let before = Instant::now();

    let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }";
    println!("<head><title>{}</title><style>{}</style></head>", &args[1], style);
    let theme = &ts.themes["base16-ocean.dark"];
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    println!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", c.r, c.g, c.b);
    let path = std::path::PathBuf::from(&args[1]);
    let ext = path.extension()
        .expect(&format!("expected file to have extension but found {:?}", &args[1]))
        .to_str()
        .expect("expected file extension to be valid unicode");
    let syn_ref = ss.find_syntax_by_extension(ext)
        .expect(&format!("failed to find syntax reference for extension {:?}", ext));
    let html = highlighted_html_for_string(&args[1], &ss, syn_ref, theme);
    println!("{}", html);
    println!("</body>");
    let after = Instant::now();
    let delta = after - before;
    eprintln!("{}", delta.as_nanos());
}
