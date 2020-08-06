use some_macros::hash_map;
use std::collections::HashMap;

fn subs_vars(s: &str, vars: HashMap<&str, &str>) -> String {
    let mut s = s.to_string();
    for (k, v) in vars {
        s = s.replace(&format!("Z{}", k), v);
    }
    s
}

fn subs_procs(mut s: String, ternary: bool) -> String {
    if ternary {
        s = s.replace("Zarity", "Ternary").replace("//if_ternary", "");
    } else {
        s = s
            .replace("Zarity", "Binary")
            .lines()
            .filter(|l| !l.trim().starts_with("//if_ternary"))
            .collect::<Vec<_>>()
            .join("\n");
    }
    s = s.split("//toggle_included").step_by(2).collect();
    s
}

fn gen(proto: &str, vars: HashMap<&str, &str>) -> (String, String) {
    // let mut s = proto.to_string();
    // for (k, v) in vars {
    //     s = s.replace(&format!("Z{}", k), v);
    // }
    // let sb = s.replace("Zarity", "Binary").replace("if_ternary_", "not_");
    // let st = s.replace("Zarity", "Ternary").replace("if_ternary_", "");
    //
    // (
    //     s.replace("Zarity", "Binary")
    //         .replace("//:if_ternary:", "//"),
    //     s.replace("Zarity", "Ternary").replace("//:if_ternary:", ""),
    // )
    let s = subs_vars(proto, vars);
    (subs_procs(s.clone(), false), subs_procs(s, true))
}

fn main() {
    println!("cargo:rerun-if-changed=src/prototype.rs");
    let s = std::fs::read_to_string("src/prototype.rs").unwrap();
    let mut spl = s.split("//sep\n");
    let _ = spl.next();
    let lib_prefix = spl.next().unwrap();
    let tern_prefix = spl.next().unwrap();
    let proto = spl.next().unwrap();
    let (side, tside) = gen(
        proto,
        hash_map!("name" => "Side", "regular" => "Right", "middle" => "Center", "inverted" => "Left"),
    );
    let (height, theight) = gen(
        proto,
        hash_map!("name" => "Height", "regular" => "Top", "middle" => "Center", "inverted" => "Bottom"),
    );
    let (depth, tdepth) = gen(
        proto,
        hash_map!("name" => "Depth", "regular" => "Front", "middle" => "Center", "inverted" => "Back"),
    );

    let s = format!(
        "{}\n{}\n{}\n{}\n\npub mod ternary {{\n{}\n{}\n{}\n{}\n}}",
        lib_prefix, side, height, depth, tern_prefix, tside, theight, tdepth
    );
    std::fs::write("src/lib.rs", s).unwrap();
}
