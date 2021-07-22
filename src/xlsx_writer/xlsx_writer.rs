use std::io::Write;

use super::{content_types::get_content_types_xml, doc_props::{app::app::get_app_xml, core::core::get_core_xml}, document::Document, rels::rels::get_rels_xml, xl::{shared_strings::get_shared_strings_xml, workbook::{rels::get_workbook_rels_xml, workbook::get_workbook_xml}, worksheet::get_worksheet_xml}};

pub struct XlsxWriter {
    document: Document,
}

impl XlsxWriter {
    pub fn new(document: Document) -> Self {
        Self { document }
    }

    pub fn write_to_file(&self, path: &std::path::Path) {
        let data = assemble_archive();
        std::fs::write(path, data).unwrap();
    }
}

fn assemble_archive() -> Vec<u8> {
    let mut buffer = Vec::new();

    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buffer));

        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        zip.add_directory("docProps", options).unwrap();

        zip.start_file("docProps/app.xml", options).unwrap();
        zip.write(get_app_xml().as_bytes()).unwrap();

        zip.start_file("docProps/core.xml", options).unwrap();
        zip.write(get_core_xml().as_bytes()).unwrap();


        zip.add_directory("_rels", options).unwrap();

        zip.start_file("_rels/.rels", options).unwrap();
        zip.write(get_rels_xml().as_bytes()).unwrap();


        zip.add_directory("xl/worksheets", options).unwrap();

        zip.start_file("xl/worksheets/sheet1.xml", options).unwrap();
        zip.write(get_worksheet_xml().as_bytes()).unwrap();

        zip.start_file("xl/workbook.xml", options).unwrap();
        zip.write(get_workbook_xml().as_bytes()).unwrap();

        zip.start_file("xl/sharedStrings.xml", options).unwrap();
        zip.write(get_shared_strings_xml().as_bytes()).unwrap();


        zip.add_directory("xl/_rels", options).unwrap();

        zip.start_file("xl/_rels/workbook.xml.rels", options).unwrap();
        zip.write(get_workbook_rels_xml().as_bytes()).unwrap();


        zip.start_file("[Content_Types].xml", options).unwrap();
        zip.write(get_content_types_xml().as_bytes()).unwrap();

        zip.finish().unwrap();
    }

    buffer
}
