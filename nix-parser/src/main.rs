use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| {s.trim().to_string()})
        .collect()
}

fn sortout(filename: &str) -> Vec<String> {
    let list: Vec<String> = read_lines(filename)
        .into_iter()
        .map(|s| s.chars().skip(2).collect::<String>())
        .filter(|line| line.ends_with(".nix"))
        .collect();
    list
}

fn inter(filename: &str, path: &str) -> Vec<String> {
    let mut final_list: Vec<String> = vec![];
    for i in sortout(filename) {
        let filepath = format!("{}/{}", path, i);
        if read_lines(&filepath)[0] == "# bundle" {
            &final_list.push(i);
            for j in sortout(&filepath) {
                &final_list.push(j);
            }
        } else {
            &final_list.push(i);
        }
    }
    final_list
}

fn output(filename: &str, path: &str) {
    for i in inter(filename, path) {
        println!("{}", i);
    }
}

fn main() {
    let path: &str = "/home/cato/nixos-config/bundles";
    let filename: &str = "xeravus.nix";
    let file = format!("{}/{}", &path, &filename);
    output(&file, &path);
}
