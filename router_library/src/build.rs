use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Gnereates a route tree from the routes directory and writes it to a file. Also ensures that
/// mod.rs files are created in each directory.
pub fn generate_routes() {
    let routes_dir = Path::new("src/routes");
    let generated_file = Path::new("src/__route_tree.rs");

    let mut route_map = BTreeMap::new();
    process_directory(routes_dir, String::new(), &mut route_map);

    let mut output = String::new();
    output.push_str("use crate::routes;\n");
    output.push_str("use router_library::router::{NodeType, RouteNode};\n\n");
    output.push_str("thread_local! {\n");
    output.push_str("    pub static ROUTES: RouteNode = {\n");
    output.push_str("        let mut root = RouteNode::new(NodeType::Static(\"\".into()));\n");

    for (route_path, handler_path) in &route_map {
        output.push_str(&format!(
            "        root.insert(\"{route_path}\", {handler_path}::handler);\n"
        ));
    }

    output.push_str("        root\n    };\n}\n");

    let mut file = File::create(generated_file).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn process_directory(dir: &Path, prefix: String, output: &mut BTreeMap<String, String>) {
    let mut mod_file = String::new();
    let mut children = vec![];

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap();
            let next_prefix = if prefix.is_empty() {
                format!("/{name}")
            } else {
                format!("{prefix}/{name}")
            };
            fs::create_dir_all(&path).unwrap();
            process_directory(&path, next_prefix, output);
            children.push(format!("pub mod {};\n", sanitize_mod(name)));
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            if stem == "mod" {
                continue;
            }

            let mod_name = sanitize_mod(stem);
            let route_path = file_to_route_path(&prefix, stem);
            let handler_path = file_to_handler_path(&prefix, stem);

            if stem.starts_with(":") || stem == "*" {
                mod_file.push_str(&format!(
                    "#[path = \"./{stem}.rs\"]\npub mod {mod_name};\n"
                ));
            } else {
                children.push(format!("pub mod {mod_name};\n"));
            }

            output.insert(route_path, handler_path);
        }
    }

    if !mod_file.is_empty() || !children.is_empty() {
        let mut contents = mod_file;
        for child in &children {
            contents.push_str(child);
        }
        let mod_path = dir.join("mod.rs");
        fs::write(mod_path, contents).unwrap();
    }
}

fn sanitize_mod(name: &str) -> String {
    match name {
        "*" => "__any".into(),
        s if s.starts_with(":") => s.trim_start_matches(":").into(),
        s => s.replace('.', "_"),
    }
}

fn file_to_route_path(prefix: &str, name: &str) -> String {
    let mut parts = vec![];
    if !prefix.is_empty() {
        parts.push(prefix.to_string());
    }
    parts.push(if name == "index" {
        "".into()
    } else if name == "*" {
        "*".into()
    } else {
        name.to_string()
    });
    parts.join("/").replace("//", "/")
}

fn file_to_handler_path(prefix: &str, name: &str) -> String {
    let mut parts: Vec<String> = prefix
        .split('/')
        .filter(|s| !s.is_empty())
        .map(sanitize_mod)
        .collect();
    parts.push(sanitize_mod(name));
    format!("routes::{}", parts.join("::"))
}
