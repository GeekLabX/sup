//! Command `source`
use crate::{registry::Registry, result::Result};
use std::cmp::Ordering;

const MAX_PACKAGE_NAME_LEN: usize = 50;

fn cap(mut name: String) -> String {
    name.push_str(&" ".repeat(MAX_PACKAGE_NAME_LEN - name.len()));
    name
}

/// Exec command `source`
pub fn exec(query: String, version: bool) -> Result<()> {
    let registry = Registry::new()?;
    let mut source = registry.source()?;
    source.sort_by(|(np, _), (nq, _)| np.partial_cmp(nq).unwrap_or(Ordering::Greater));

    println!(
        "{}",
        if query.is_empty() {
            source
                .iter()
                .map(|(name, ver)| {
                    if version {
                        name.to_string()
                    } else {
                        format!("{} - {}", cap(name.to_string()), ver)
                    }
                })
                .collect::<Vec<String>>()
        } else {
            source
                .iter()
                .filter(|(name, _)| name.contains(&query))
                .map(|(name, ver)| {
                    if version {
                        name.to_string()
                    } else {
                        format!("{} - {}", cap(name.to_string()), ver)
                    }
                })
                .collect::<Vec<String>>()
        }
        .join("\n")
    );
    Ok(())
}
