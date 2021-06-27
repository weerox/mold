use std::convert::TryFrom;

use crate::cursor::Cursor;
use crate::error::ParseError;

#[derive(Debug, PartialEq)]
struct Content<'a> {
    children: Vec<Node<'a>>,
}

#[derive(Debug, PartialEq)]
enum Node<'a> {
    Tag(Tag<'a>),
    Text(&'a str),
}

#[derive(Debug, PartialEq)]
struct Tag<'a> {
    name: &'a str,
    // attributes: HashMap<K,V>
    content: Content<'a>,
}

impl<'input> TryFrom<&'input str> for Content<'input> {
    type Error = ParseError;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        let cursor = &mut Cursor::new(input);
        let content = build_content(cursor);

        // If we aren't at the end of input, then something has gone wrong.
        if cursor.first() != None {
            let start = cursor.byte_offset();

            let tag_type = match find_tag(cursor) {
                None => panic!("Something happend and I don't know what"),
                Some(t) => t,
            };

            if start != cursor.byte_offset() {
                panic!("We really should have found a tag right after");
            }

            if tag_type == TagType::Closing {
                return Err(ParseError::NoOpeningTag);
            }
        }

        content
    }
}

fn build_content<'a>(cursor: &mut Cursor<'a>) -> Result<Content<'a>, ParseError> {
    let mut content = Content {
        children: Vec::new(),
    };

    loop {
        if let Some(text) = build_text(cursor)? {
            content.children.push(Node::Text(text));
        }

        let tag = match find_tag(cursor) {
            None => break, // return content
            Some(t) => t,
        };

        if tag == TagType::Closing {
            break; // return content
        } else {
            let tag = build_tag(cursor)?;
            content.children.push(Node::Tag(tag));
        }
    }

    Ok(content)
}

fn build_text<'a>(cursor: &mut Cursor<'a>) -> Result<Option<&'a str>, ParseError> {
    let start = cursor.byte_offset();

    find_tag(cursor);

    let end = cursor.byte_offset();

    let text = &cursor.input()[start..end];

    if !text.is_empty() {
        Ok(Some(text))
    } else {
        Ok(None)
    }
}

// Returns a Tag
// This function must only be called when the cursor is positioned right before a tag.
fn build_tag<'a>(cursor: &mut Cursor<'a>) -> Result<Tag<'a>, ParseError> {
    #[cfg(debug_assertions)]
    let pos = cursor.position();

    let tag_t = find_tag(cursor);

    debug_assert_eq!(pos, cursor.position());

    if let Some(tag_t) = tag_t {
        match tag_t {
            TagType::Opening => {
                skip_sign(cursor);

                let name = parse_tag_name(cursor);

                // let attributes = parse_tag_attributes();
                
                skip_sign(cursor);

                let content = build_content(cursor)?;

                // TODO we should be right before the closing tag here
                match find_tag(cursor) {
                    None => return Err(ParseError::NoClosingTag),
                    Some(t) => {
                        if t == TagType::Closing {
                            skip_sign(cursor);

                            let closing_name = parse_tag_name(cursor);

                            skip_sign(cursor);

                            if closing_name != name {
                                return Err(ParseError::InvalidClosingTag);
                            }
                        }
                    },
                }
                
                Ok(Tag {
                    name: name,
                    content: content,
                })
            },
            TagType::Closing => {
                unreachable!();
            },
            TagType::SelfClosing => {
                let name = parse_tag_name(cursor);

                skip_sign(cursor);

                Ok(Tag {
                    name: name,
                    content: Content {
                        children: Vec::new(),
                    },
                })
            },
        }
    } else {
        panic!("No tags left, you lied to me!")
    }
}

fn parse_tag_name<'a>(cursor: &mut Cursor<'a>) -> &'a str {
    cursor.skip_while(|c| c.is_ascii_whitespace());

    cursor.take_while(|c| c.is_ascii_alphabetic())
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TagType {
    Opening,
    Closing,
    SelfClosing,
}

// Put the cursor before the next tag
fn find_tag(cursor: &mut Cursor) -> Option<TagType> {
    let first = find_sign(cursor);

    let mut c = cursor.clone();

    c.skip(2);

    let second = find_sign(&mut c);

    match (first, second) {
        (None, None) => None,
        (None, _) => unreachable!(),
        (s, None) => panic!("No matching sign for {:?}", s),
        (Some(a), Some(b)) => match (a, b) {
            (Sign::Opening, Sign::Opening) => Some(TagType::Opening),
            (Sign::Closing, Sign::Closing) => Some(TagType::Closing),
            (Sign::Opening, Sign::Closing) => Some(TagType::SelfClosing),
            _ => panic!("Invalid combination of signs"),
        },
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Sign {
    Opening,
    Closing,
}

// Puts the cursor right before the next sign and returns the type of sign.
fn find_sign(cursor: &mut Cursor) -> Option<Sign> {
    loop {
        match (cursor.first(), cursor.second()) {
            (Some(a), Some(b)) => match (a, b) {
                ('<', '<') => return Some(Sign::Opening),
                ('>', '>') => return Some(Sign::Closing),
                (_, '<') | (_, '>') => cursor.skip(1),
                (_, _) => cursor.skip(2),
            },
            (Some(_), None) => {
                cursor.skip(1);
                break;
            },
            _ => break,
        }
    }

    None
}

// Put the cursor after the next sign
fn skip_sign(cursor: &mut Cursor) {
    find_sign(cursor);
    cursor.skip(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_signs() {
        let mut cursor = Cursor::new("abc");

        let sign = find_sign(&mut cursor);

        // When there are no signs to be found,
        // the cursor should be at the end of the input
        // and the returned value should be None.
        assert!(sign.is_none());
        assert_eq!(cursor.first(), None);
    }

    #[test]
    fn only_one_opening_sign() {
        let mut cursor = Cursor::new("<<");

        let sign = find_sign(&mut cursor);

        assert_eq!(sign, Some(Sign::Opening));
        assert_eq!((cursor.first(), cursor.second()), (Some('<'), Some('<')));
        assert_eq!(cursor.input().chars().count() - cursor.position(), 2);
    }

    #[test]
    fn only_two_opening_signs() {
        let mut cursor = Cursor::new("<<<<");

        let sign = find_sign(&mut cursor);

        assert_eq!(sign, Some(Sign::Opening));
        assert_eq!((cursor.first(), cursor.second()), (Some('<'), Some('<')));
        assert_eq!(cursor.position(), 0);

        cursor.skip(2);

        let sign = find_sign(&mut cursor);

        assert_eq!(sign, Some(Sign::Opening));
        assert_eq!((cursor.first(), cursor.second()), (Some('<'), Some('<')));
        assert_eq!(cursor.position(), 2);
    }

    #[test]
    fn one_opening_sign() {
        let mut cursor = Cursor::new("abc<<def");

        let sign = find_sign(&mut cursor);

        assert_eq!(sign, Some(Sign::Opening));
        assert_eq!((cursor.first(), cursor.second()), (Some('<'), Some('<')));
        assert_eq!(cursor.position(), 3);
    }

    #[test]
    fn find_tag_moves_to_end_without_any_tags_left() {
        let mut cursor = Cursor::new("<<abc<<def");
        
        skip_sign(&mut cursor);
        skip_sign(&mut cursor);

        let tag_t = find_tag(&mut cursor);

        assert_eq!(tag_t, None);
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn only_text_content() {
        let content = Content::try_from("abc");

        assert!(content.is_ok());

        let content = content.unwrap();

        assert_eq!(content.children.len(), 1);
        assert_eq!(content.children[0], Node::Text("abc"));
    }

    #[test]
    fn one_tag_with_only_text_content() {
        let content = Content::try_from("<<foo<<bar>>foo>>");

        assert!(content.is_ok());

        let content = content.unwrap();

        let mut eq = Content {
            children: Vec::new(),
        };

        eq.children.push(Node::Tag(Tag {
            name: "foo",
            content: Content {
                children: vec![Node::Text("bar")],
            }
        }));

        assert_eq!(content, eq);
    }

    #[test]
    fn unclosed_tag() {
        let content = Content::try_from("<<foo<<bar");

        assert_eq!(content, Err(ParseError::NoClosingTag));
    }

    #[test]
    fn closing_tag_that_does_not_match() {
        let content = Content::try_from("<<foo<<bar>>baz>>");

        assert_eq!(content, Err(ParseError::InvalidClosingTag));
    }

    #[test]
    fn only_closing_tag() {
        let content = Content::try_from(">>foo>>");

        assert_eq!(content, Err(ParseError::NoOpeningTag));
    }
}
