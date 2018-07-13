#[macro_use]
extern crate failure;
extern crate pulldown_cmark;
extern crate walkdir;

#[cfg(test)]
mod tests {
    use pulldown_cmark::{Event, Parser, Tag};
    use std::borrow::Cow;
    use std::path::{Path, PathBuf};
    use std::{fs, mem};
    use walkdir::WalkDir;

    type Result<T> = ::std::result::Result<T, ::failure::Error>;

    #[derive(Debug)]
    struct MarkdownContent {
        path: PathBuf,
        body: String,
    }

    fn collect_markdown_files(root: impl AsRef<Path>) -> Result<Vec<MarkdownContent>> {
        let mut md_contents = vec![];

        for entry in WalkDir::new(root) {
            let entry = entry?;
            if entry.file_type().is_file() {
                md_contents.push(MarkdownContent {
                    path: entry.path().to_owned(),
                    body: fs::read_to_string(entry.path())?,
                });
            }
        }

        Ok(md_contents)
    }

    #[derive(Debug)]
    struct CodeBlock<'a> {
        annotation: Cow<'a, str>,
        texts: Vec<Cow<'a, str>>,
    }

    impl<'a> CodeBlock<'a> {
        fn text(&self) -> Cow<'a, str> {
            match self.texts.len() {
                0 => "".into(),
                1 => self.texts[0].clone(),
                _ => self.texts
                    .iter()
                    .fold(String::new(), |mut acc, text| {
                        acc.push_str(&*text);
                        acc
                    })
                    .into(),
            }
        }
    }

    fn collect_code_blocks(content: &MarkdownContent) -> Result<Vec<CodeBlock>> {
        let mut blocks = vec![];

        let mut annotation = None;
        let mut texts = vec![];
        for event in Parser::new(&content.body) {
            match event {
                Event::Start(Tag::CodeBlock(annot)) => {
                    if annotation.is_some() {
                        bail!("nested code block")
                    }
                    annotation = Some(annot);
                }
                Event::End(Tag::CodeBlock(annot)) => {
                    if annotation.as_ref().map_or(false, |a| *a == &*annot) {
                        let annotation = annotation.take().expect("empty annotation");
                        let texts = mem::replace(&mut texts, vec![]);
                        blocks.push(CodeBlock { annotation, texts })
                    } else {
                        bail!("unexpeted end tag");
                    }
                }
                Event::Text(t) => {
                    if annotation.is_some() {
                        texts.push(t)
                    }
                }
                _ => {}
            }
        }

        Ok(blocks)
    }

    #[test]
    fn main() {
        let md_contents =
            collect_markdown_files(concat!(env!("CARGO_MANIFEST_DIR"), "/../src")).unwrap();
        for content in md_contents {
            let blocks = collect_code_blocks(&content).unwrap();
            for block in blocks {
                println!("annotation: {}", block.annotation);
                println!("text:");
                println!("===============");
                println!("{}", block.text());
                println!("===============\n");
            }
        }
    }
}
