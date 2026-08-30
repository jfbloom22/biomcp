use std::collections::BTreeSet;

use serde_json::{Map, json};

pub(super) fn typed_get_schema(schema: &mut rmcp::schemars::Schema) {
    let entities = [
        "author",
        "gene",
        "article",
        "disease",
        "diagnostic",
        "pgx",
        "trial",
        "variant",
        "drug",
        "pathway",
        "protein",
        "adverse-event",
    ];
    let sections = entities
        .iter()
        .filter(|&&entity| entity != "author")
        .flat_map(|&entity| crate::cli::list::catalog::sections(entity))
        .collect::<BTreeSet<_>>();
    let properties = Map::from_iter([
        ("entity".into(), json!({"type":"string","enum":entities})),
        (
            "id".into(),
            json!({"type":"string","minLength":1,"maxLength":512}),
        ),
        ("json".into(), json!({"type":"boolean","default":false})),
        (
            "sections".into(),
            json!({"type":"array","maxItems":16,"items":{"enum":sections}}),
        ),
        (
            "assembly".into(),
            json!({"type":"string","enum":["grch37","hg19","grch38","hg38"]}),
        ),
    ]);
    *schema = serde_json::from_value(json!({
        "type":"object",
        "additionalProperties":false,
        "properties":properties,
        "required":["entity","id"]
    }))
    .expect("valid portable typed get schema");
}
