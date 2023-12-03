mod parsing;
use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};
use nom::{combinator::all_consuming, Finish};
use parsing::{parse_line, Command, Entry, Line};

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let lines = include_str!("../../../inputs/7-no-space-left.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut tree = Tree::<FsEntry>::new();
    let root = tree.insert(
        Node::new(FsEntry {
            path: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;
    let mut curr = root;

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => (),
                Command::Cd(path) => match path.as_str() {
                    "/" => (),
                    ".." => curr = tree.get(&curr)?.parent().unwrap().clone(),
                    _ => {
                        let node = Node::new(FsEntry {
                            path: path.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(path) => (),
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry {
                        path: path.clone(),
                        size,
                    });
                    tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                }
            },
        }
    }
    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum::<u64>();
    dbg!(sum);
    Ok(())
}
