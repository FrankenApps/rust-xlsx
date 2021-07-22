pub fn get_shared_strings_xml() -> String {
    String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <sst count="%d" uniqueCount="%d" xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
            <si><t>Test</t></si>
        </sst>
        "#
    )
}