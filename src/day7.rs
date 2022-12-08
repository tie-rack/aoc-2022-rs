use either::{Either, Left, Right};

type DiskObject<'a> = Either<File<'a>, Dir<'a>>;

#[derive(Clone, Copy, Debug, PartialEq)]
struct File<'a> {
    name: &'a str,
    bytes: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Dir<'a> {
    name: &'a str,
    contents: Vec<DiskObject<'a>>,
}

trait MadeOfBytes {
    fn size(&self) -> usize;
}

impl MadeOfBytes for DiskObject<'_> {
    fn size(&self) -> usize {
        match self {
            Left(f) => f.bytes,
            Right(dir) => dir.contents.iter().map(MadeOfBytes::size).sum(),
        }
    }
}

trait Walkable {
    fn walk(&self) -> Vec<DiskObject>;
}

impl Walkable for DiskObject<'_> {
    fn walk(&self) -> Vec<DiskObject> {
        match self {
            Left(f) => vec![Left(*f)],
            Right(dir) => {
                let mut contents: Vec<DiskObject> =
                    dir.contents.iter().flat_map(Walkable::walk).collect();

                contents.push(Right(dir.clone()));
                contents
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_disk<'a>() -> DiskObject<'a> {
        Right(Dir {
            name: "",
            contents: vec![
                Right(Dir {
                    name: "a",
                    contents: vec![
                        Right(Dir {
                            name: "e",
                            contents: vec![Left(File {
                                name: "i",
                                bytes: 584,
                            })],
                        }),
                        Left(File {
                            name: "f",
                            bytes: 29116,
                        }),
                        Left(File {
                            name: "g",
                            bytes: 2557,
                        }),
                        Left(File {
                            name: "h.lst",
                            bytes: 62596,
                        }),
                    ],
                }),
                Left(File {
                    name: "b.txt",
                    bytes: 14848514,
                }),
                Left(File {
                    name: "c.dat",
                    bytes: 8504156,
                }),
                Right(Dir {
                    name: "d",
                    contents: vec![
                        Left(File {
                            name: "j",
                            bytes: 4060174,
                        }),
                        Left(File {
                            name: "d.log",
                            bytes: 8033020,
                        }),
                        Left(File {
                            name: "d.ext",
                            bytes: 5626152,
                        }),
                        Left(File {
                            name: "k",
                            bytes: 7214296,
                        }),
                    ],
                }),
            ],
        })
    }

    #[test]
    fn test_disk_object_size() {
        assert_eq!(
            42,
            Left(File {
                name: "a",
                bytes: 42
            })
            .size()
        );
        assert_eq!(48381165, example_disk().size());
    }

    #[test]
    fn test_small_dirs() {
        assert_eq!(
            95437_usize,
            example_disk()
                .walk()
                .iter()
                .filter(|obj| obj.is_right())
                .map(|obj| obj.size())
                .filter(|size| size < &100000)
                .sum()
        );
    }
}
