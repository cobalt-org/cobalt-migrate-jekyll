use std::ffi;
use std::path;

use cobalt::cobalt_model;
use cobalt::cobalt_model::files;
use cobalt::jekyll_model;

use error::*;

pub fn convert_from_jk(source: &path::Path, dest: &path::Path) -> Result<()> {
    if dest.is_file() {
        bail!("Destination must be a directory");
    } else if source.is_file() {
        let rel_src = source
            .file_name()
            .expect("file_name exists since its a file");
        let dest_file = dest.join(rel_src);
        convert_document_file(source, &dest_file)
    } else if source.is_dir() {
        for file in source.read_dir()? {
            if let Ok(file) = file {
                let file_path = file.path();
                let ext = file_path.extension().unwrap_or_else(|| ffi::OsStr::new(""));
                if file_path.is_file() {
                    if ext == "md" || ext == "markdown" {
                        let rel_src = file_path
                            .strip_prefix(source)
                            .expect("file was found under the root");
                        let dest_file = dest.join(rel_src);
                        convert_document_file(&file_path, &dest_file)?
                    } else {
                        warn!("unsupported file extension")
                    }
                } else {
                    warn!("sub directory parsing is not supported yet")
                }
            }
        }
        Ok(())
    } else {
        bail!("Unrecognized source");
    }
}

fn convert_document_file(source_file: &path::Path, dest_file: &path::Path) -> Result<()> {
    let doc = files::read_file(source_file)?;
    let doc = jekyll_model::DocumentBuilder::parse(&doc)?;
    let doc: cobalt_model::DocumentBuilder<cobalt_model::FrontmatterBuilder> = doc.into();
    let doc = doc.to_string();
    files::write_document_file(&doc, dest_file)?;
    Ok(())
}
