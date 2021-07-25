pub fn get_worksheet_xml() -> String {
    String::from(
        r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
    <sheetData>
        <row r="1">
            <c r="A1" t="s" s="2">
                <v>0</v>
            </c>
        </row>
    </sheetData>
</worksheet>
        "#
    )
}