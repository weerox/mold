use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::fs::create_dir;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

use crate::content::Content;
use crate::content::Node;
use crate::content::Tag;

// Takes all templates and makes all extended templates stand-alone.
pub fn flatten_templates<D: AsRef<Path>, T: AsRef<Path>>(templates: &Vec<D>, tmp: &T) {
    // The 'file_content' is only used to make sure the Strings is owned by something.
    let mut file_content: Vec<String> = Vec::new(); 
    let mut content: Vec<Content> = Vec::new();
    let mut filenames: Vec<String> = Vec::new();

    for dir in templates {
        let files = dir.as_ref().read_dir().unwrap();

        for f in files {
            let f = f.unwrap();

            let ft = f.file_type().unwrap();

            if ft.is_file() {
                filenames.push(f.path().file_stem().unwrap().to_owned().into_string().unwrap());
                let fc = read_to_string(f.path()).unwrap();
                file_content.push(fc);
            }
        }
    }

    for fc in &file_content {
        let c = Content::try_from(fc.as_ref()).unwrap();
        content.push(c);
    }

    let edges = create_edges(filenames.iter().map(|s| s.as_ref()).zip(&content).collect());

    let hiers = build_hierarchy(edges);

    // TODO Make this a HashMap
    let mut flattened: Vec<(&str, Content)> = Vec::new();

    fn recursive<'a, 'b>(
        flattened: &mut Vec<(&'b str, Content<'a>)>,
        content: &Vec<(&str, &Content<'a>)>,
        hier: &'b Hierarchy
    ) {
        for child in &hier.children {
            let (_, p) =
                flattened.iter()
                .find(|(n, _)| n == &hier.name)
                .unwrap();
            let (_, c) =
                content.iter()
                .find(|(n, _)| n == &child.name)
                .unwrap();

            // "Remove" the outermost tag, which is just the name of the parent
            debug_assert!(matches!(&c.children[0], Node::Tag(t) if &t.name == &hier.name));
            let c = match &c.children[0] {
                Node::Tag(t) => &t.content,
                _ => panic!(),
            };

            let f = flatten(p, c);
            flattened.push((child.name, f));

            recursive(flattened, content, child);
        }
    }

    let fnc: Vec<(&str, &Content)> = filenames.iter().map(|s| s.as_ref()).zip(content.iter()).collect();
    for hier in &hiers {
        let &(n, c) = 
            fnc.iter()
            .find(|(n, _)| n == &hier.name)
            .unwrap();

        // All root templates are already flattened.
        flattened.push((n, c.clone()));

        recursive(&mut flattened, &fnc, &hier);
    }

    let templ = tmp.as_ref().join("templates/");
    create_dir(&templ).unwrap();

    for (name, content) in flattened {
        let mut file = File::create(templ.join(name)).unwrap();
        write!(&mut file, "{}", content).unwrap();
    }
}

fn flatten<'a>(parent: &Content<'a>, child: &Content<'a>) -> Content<'a> {
    let mut content = parent.clone();

    let mut tlt: HashMap<&str, &mut Tag> = HashMap::new();
    for node in &mut content.children {
        match node {
            Node::Tag(tag) => tlt.insert(tag.name, tag),
            Node::Text(_) => continue,
        };
    }

    for node in &child.children {
        match node {
            Node::Tag(tag) => {
                if tlt.contains_key(tag.name) {
                    let t = tlt.get_mut(tag.name).unwrap();
                    t.content = tag.content.clone();
                }
            },
            Node::Text(_) => continue,
        }
    }
    
    content
}

fn create_edges<'a, 'b>(
    content: Vec<(&'b str, &Content<'a>)>
) -> Vec<(Option<&'a str>, &'b str)> {
    let mut v = Vec::new();

    for (filename, c) in &content {
        // Many editors insert a newline at the end of a file,
        // which will create a Text node at the end of the Content tree.
        let has_trailing_whitespace: bool = if c.children.len() == 2 {
            let child = &c.children[1];
            if let Node::Text(text) = child {
                if text.chars().all(|c| c.is_ascii_whitespace()) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if c.children.len() == 1 || has_trailing_whitespace {
            if let Node::Tag(tag) = &c.children[0] {
                v.push((Some(tag.name), *filename));
                continue;
            }
        }

        v.push((None, *filename));
    }

    v
}

#[derive(Debug)]
struct Hierarchy<'a> {
    name: &'a str,
    children: Vec<Hierarchy<'a>>,
}

// Takes a list of (parent, child) tuples and converts it into one or more hierarchies.
// TODO This currently assumes that the input is well-formed.
fn build_hierarchy<'a>(edges: Vec<(Option<&'a str>, &'a str)>) -> Vec<Hierarchy<'a>> {
    let mut roots = Vec::new();

    let mut nodes: HashMap<&'a str, Vec<Hierarchy<'a>>> = HashMap::new();

    for &(parent, child) in &edges {
        if let Some(parent) = parent {
            if nodes.contains_key(parent) {
                nodes.get_mut(parent).unwrap().push(Hierarchy {
                    name: child,
                    children: Vec::new(),
                });
            } else {
                let mut v = Vec::new();
                v.push(Hierarchy {
                    name: child,
                    children: Vec::new(),
                });

                nodes.insert(parent, v);
            }
        } else {
            roots.push(Hierarchy {
                name: child,
                children: Vec::new(),
            });
        }
    }

    fn add_children<'a>(
        nodes: &mut HashMap<&'a str, Vec<Hierarchy<'a>>>,
        h: &mut Hierarchy<'a>
    ) {
        let mut v = match nodes.remove(h.name) {
            Some(v) => v,
            None => return,
        };

        while !v.is_empty() {
            h.children.push(v.pop().unwrap());
            add_children(nodes, h.children.last_mut().unwrap());
        }
    }

    for root in &mut roots {
        add_children(&mut nodes, root);
    }

    roots
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_root_node() {
        let nodes = vec![(None, "foo")];

        let hier = build_hierarchy(nodes);

        assert_eq!(hier.len(), 1);
        assert_eq!(hier[0].name, "foo");
        assert_eq!(hier[0].children.len(), 0);
    }

    #[test]
    fn one_root_with_one_child() {
        let nodes = vec![(None, "foo"), (Some("foo"), "bar")];

        let hier = build_hierarchy(nodes);

        assert_eq!(hier.len(), 1);
        assert_eq!(hier[0].name, "foo");
        assert_eq!(hier[0].children.len(), 1);
        assert_eq!(hier[0].children[0].name, "bar");
    }

    #[test]
    fn one_root_with_two_children() {
        let nodes = vec![(None, "foo"), (Some("foo"), "bar"), (Some("foo"), "baz")];

        let hier = build_hierarchy(nodes);

        assert_eq!(hier.len(), 1);
        assert_eq!(hier[0].name, "foo");
        assert_eq!(hier[0].children.len(), 2);
    }

    #[test]
    fn one_root_with_two_level_chilren() {
        let nodes = vec![(None, "foo"), (Some("foo"), "bar"), (Some("bar"), "baz")];

        let hier = build_hierarchy(nodes);

        assert_eq!(hier.len(), 1);
        assert_eq!(hier[0].name, "foo");
        assert_eq!(hier[0].children.len(), 1);
        assert_eq!(hier[0].children[0].children.len(), 1);
    }

    #[test]
    fn two_roots() {
        let nodes = vec![(None, "foo"), (None, "bar")];

        let hier = build_hierarchy(nodes);

        assert_eq!(hier.len(), 2);
    }

    #[test]
    fn ignores_trailing_newline() {
        let t_root = "This is the root template.\n";
        let t_child = "<<root<<This template extends root>>root>>\n";

        let pairs = vec![("root", Content::try_from(t_root).unwrap()), ("child", Content::try_from(t_child).unwrap())];

        let edges = create_edges(pairs.iter().map(|(a, b)| (*a, b)).collect());

        assert_eq!(edges, vec![(None, "root"), (Some("root"), "child")]);
    }

    #[test]
    fn simple_template_flattening() {
        use std::fs::read_to_string;
        use tempfile::tempdir;

        let t_base = "\
<!DOCTYPE html>
<html>
 <head>
  <title><<title<<My Website>>title>></title>
 </head>
 <body>
  <!-- Header here -->
  <main>
   <<content>>
  </main>
  <!-- Footer here -->
 </body>
</html>
";

    let t_post = "\
<<base<<
 <<content<<
  <article>
   <h1><<title>></h1>
   <time datetime=\"<<time>>\"><<time>></time>
   <<content>>
  </article>
 >>content>>
>>base>>
";

        let out = tempdir().unwrap();

        let mut file = File::create(out.path().join("base.html")).unwrap();
        write!(file, "{}", t_base).unwrap();

        let mut file = File::create(out.path().join("post.html")).unwrap();
        write!(file, "{}", t_post).unwrap();

        let dirs = vec![out];
        let tmp = tempdir().unwrap();
        flatten_templates(&dirs, &tmp);

        let f_base = t_base;

        let f_post = "\
<!DOCTYPE html>
<html>
 <head>
  <title><<title<<My Website>>title>></title>
 </head>
 <body>
  <!-- Header here -->
  <main>
   <<content<<
  <article>
   <h1><<title>></h1>
   <time datetime=\"<<time>>\"><<time>></time>
   <<content>>
  </article>
 >>content>>
  </main>
  <!-- Footer here -->
 </body>
</html>
";

        let templ = tmp.path().join("templates/");
        assert_eq!(read_to_string(templ.join("base")).unwrap(), f_base);
        assert_eq!(read_to_string(templ.join("post")).unwrap(), f_post);
    }
}
