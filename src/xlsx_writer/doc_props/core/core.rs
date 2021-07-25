pub fn get_core_xml() -> String {
    String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
	<dcterms:created xsi:type="dcterms:W3CDTF">2000-01-01T00:00:00.00Z</dcterms:created>
	<dc:creator>Me</dc:creator>
</cp:coreProperties>
        "#
    )
}