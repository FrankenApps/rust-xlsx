use xlsx_writer::{
    document::Document, document_properties::DocumentProperties, xlsx_writer::XlsxWriter,
};

pub mod xlsx_writer {
    pub mod content_types;

    pub mod doc_props {
        pub mod app {
            pub mod app;
        }

        pub mod core {
            pub mod core;
        }
    }

    pub mod document;

    pub mod document_properties;

    pub mod rels {
        pub mod rels;
    }

    pub mod xl {
        pub mod shared_strings;

        pub mod workbook {
            pub mod rels;

            pub mod workbook;
        }

        pub mod worksheet;
    }

    pub mod xlsx_writer;
}

fn main() {
    let writer = XlsxWriter::new(Document::new(DocumentProperties::new(
        String::from("Ferdinand Schäffler"),
        String::from("None"),
        String::from("2000-01-01T00:00:00.00Z"),
    )));

    writer.write_to_file(std::path::Path::new("test.xlsx"));
}
