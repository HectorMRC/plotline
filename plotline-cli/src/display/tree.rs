use std::fmt::Display;

/// Provides a method to display the inner data into a tree.
pub struct DisplayTree<'a, T> {
    chunks: Vec<&'a [T]>,
    // chunk_name_fn: Option<ChunkNameFn>,
}

impl<'a, T> Display for DisplayTree<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.chunks.is_empty() {
            return Ok(());
        }

        write!(f, "")
    }
}

impl<'a, T> DisplayTree<'a, T>
where
    T: Eq,
{
    pub fn new(items: &'a [T]) -> Self {
        Self {
            chunks: items.chunk_by(T::eq).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::display::DisplayTree;

    #[test]
    fn display_tree() {
        struct Test<'a> {
            name: &'a str,
            items: Vec<&'a str>,
            want: &'a str,
        }

        vec![
            Test {
                name: "empty list should display nothing",
                items: vec![],
                want: "",
            },
            // Test {
            //     name: "list with only one element should display a single bullet",
            //     items: vec!["this is a 'bullet' in the tree"],
            //     want: &format!("⭘    this is a 'bullet' in the tree"),
            // },
            Test {
                name: "list with two consecutive element should display a connect bullet list",
                items: vec![
                    "this is the first 'bullet' in the tree",
                    "this is the second 'bullet' in the tree",
                ],
                want: &format!(
                    concat!(
                        "⭘    this is the second 'bullet' in the tree\n",
                        "\u{007C}\n",
                        "⭘    this is the first 'bullet' in the tree\n"
                    )
                ),
            },
        ]
        .into_iter()
        .for_each(|test| {
            println!(">>>>>>>>>>>>>>>");
            println!("{}", concat!(
                "⭘    this is the second 'bullet' in the tree\n",
                "\u{007C}\n",
                "⭘    this is the first 'bullet' in the tree\n"
            ));

            let got = DisplayTree::new(test.items.as_slice()).to_string();
            assert_eq!(test.want, got, "{}", test.name);
        });
    }
}
