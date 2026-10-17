//! Caller deliberately renames the dependency and shadows the helper.
#![allow(unused_variables, unreachable_code)]
#[cfg(test)]
mod tests {
    fn format_event(_: &str) -> String {
        "wrong caller helper".into()
    }
    #[test]
    fn exported_macro_resolves_public_definition_path() {
        let name = "external";
        let actual: String = renamed_events::ch22_event!(name);
        assert_eq!(actual, "event:external");
        assert_eq!(name, "external");
        assert_eq!(format_event(name), "wrong caller helper");
    }
}
