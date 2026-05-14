use std::collections::HashMap;
use std::collections::HashSet;

pub struct Platform {
    pub platform: String,
    pub date: String,
    pub note: String,
    pub link: String,
    pub latest: String,
    pub stable: String,

    pub items: Vec<Item>,
    pub sha256: Vec<Vec<String>>,
}

pub struct Item {
    pub id: String,
    pub name: String,
    pub group: String,
    pub author: String,
    pub inst: String,
    pub last: String,
    pub href: String,
}

pub fn parse_aht(lines: String) -> Vec<HashMap<String, String>> {
    let mut list = Vec::new();
    let mut keys = HashSet::new();
    let mut map: HashMap<String, String> = HashMap::new();
    let mut key = "".to_string();

    for line in lines.lines() {
        if line.starts_with(";") {
            continue;
        } else if line.is_empty() {
            continue;
        } else if line.starts_with("%") {
            key = line.trim_start_matches("%").to_string();
            if keys.contains(&key) {
                keys.clear();
                if !map.is_empty() {
                    list.push(map);
                }
                map = HashMap::new();
            }
            keys.insert(key.clone());
        } else {
            if map.contains_key(&key) {
                let new_val = map.get(&key).cloned().unwrap_or_default() + "\n" + line;
                map.insert(key.clone(), new_val.trim().to_string());
            } else {
                map.insert(key.clone(), line.trim().to_string());
            }
        }
    }

    if !map.is_empty() {
        list.push(map);
    }
    list
}

pub fn parse_hsp_index(contents: String) -> Platform {
    let mut items = Vec::new();
    let parsed = parse_aht(contents);

    for item in parsed.iter() {
        if !item.contains_key("index") {
            continue; // ヘッダーやsha256ブロックをスキップ
        }

        let index: Vec<String> = item["index"].split("\n").map(|f| f.to_string()).collect();
        items.push(Item {
            id: index[0].clone(),
            name: index[1].clone(),
            group: item.get("group").cloned().unwrap_or_default(),
            author: item.get("author").cloned().unwrap_or_default(),
            inst: item.get("inst").cloned().unwrap_or_default(),
            last: item.get("last").cloned().unwrap_or_default(),
            href: item.get("href").cloned().unwrap_or_default(),
        });
    }

    let sha256: Vec<Vec<String>> = parsed[parsed.len() - 1]
        .get("sha256")
        .map(|s| {
            s.split("\n")
                .map(|f| f.split(":").map(|f| f.to_string()).collect())
                .collect()
        })
        .unwrap_or_default();

    Platform {
        platform: parsed[0].get("platform").cloned().unwrap_or_default(),
        date: parsed[0].get("date").cloned().unwrap_or_default(),
        note: parsed[0].get("note").cloned().unwrap_or_default(),
        link: parsed[0].get("link").cloned().unwrap_or_default(),
        latest: parsed[0].get("latest").cloned().unwrap_or_default(),
        stable: parsed[0].get("stable").cloned().unwrap_or_default(),
        items,
        sha256,
    }
}
