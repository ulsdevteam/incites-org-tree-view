use anyhow::Result;
use itertools::join;
use std::collections::HashMap;
use std::io;

struct Org {
    name: String,
    children: Vec<String>,
}

fn main() -> Result<()> {
    let mut csv = csv::Reader::from_reader(io::stdin());
    let mut orgs = HashMap::new();

    for record in csv.records() {
        let record = record?;
        orgs.insert(
            record[0].to_owned(),
            Org {
                name: record[1].to_owned(),
                children: Vec::new(),
            },
        );
        if let Some(parent) = orgs.get_mut(&record[2]) {
            parent.children.push(record[0].to_owned());
        }
    }

    let page = format!(
        "<html>
            <head>
                <title>InCites Organization Tree View</title>
            </head>
            <body>
                <ul>
                    {}
                </ul>
            </body>
        </html>",
        build_tree(&orgs, &orgs["0"])
    );

    print!("{}", page);

    Ok(())
}

fn build_tree(all_orgs: &HashMap<String, Org>, org: &Org) -> String {
    if org.children.is_empty() {
        format!("<li>{}</li>", org.name)
    } else {
        format!(
            "<li>
                <details>
                    <summary>{}</summary>
                    <ul>
                        {}
                    </ul>
                </details>
            </li>",
            org.name,
            join(org.children.iter().map(|child| build_tree(all_orgs, &all_orgs[child])), "\r\n")
        )
    }
}
