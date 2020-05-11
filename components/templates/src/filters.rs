use std::collections::HashMap;
use std::hash::BuildHasher;

use base64::{decode, encode};
use pulldown_cmark as cmark;
use regex::{Captures, Regex};
use tera::{to_value, try_get_value, Error as TeraError, Result as TeraResult, Value};

pub fn replace_all<S: BuildHasher>(
    value: &Value,
    args: &HashMap<String, Value, S>,
) -> TeraResult<Value> {
    let s = try_get_value!("replace_all", "value", String, value);

    let regex = match args.get("regex") {
        Some(val) => try_get_value!("replace_all", "regex", String, val),
        None => return Err(TeraError::msg("Filter `replace_all` expected an arg called `regex`")),
    };

    let rep = match args.get("rep") {
        Some(val) => try_get_value!("replace_all", "rep", String, val),
        None => return Err(TeraError::msg("Filter `replace_all` expected an arg called `rep`")),
    };

    let regex = match Regex::new(&regex) {
        Ok(regex) => regex,
        Err(err) => {
            return Err(TeraError::msg(format!(
                "Filter `replace_all`: Invalid regular expression: {}",
                err
            )));
        }
    };

    Ok(to_value(&regex.replace_all(&s, &*rep)).unwrap())
}

lazy_static::lazy_static! {
    // &lt; and &gt; because it’s normally already Markdown-processed.
    static ref BIBLE_REFS: Regex = Regex::new("★(.*?)(?: (?:&lt;|<)(.*?)(?:&gt;|>))?★").unwrap();
    // Same as BIBLE_REFS, but with the ★ wrapping changed to ^ and $.
    static ref BIBLE_REF: Regex = Regex::new("^(.*?)(?: (?:&lt;|<)(.*?)(?:&gt;|>))?$").unwrap();

    static ref REFERENCE_PARSER: Regex = Regex::new(r#"^((?:[123] )?[A-Za-z ]+)(?: (\d+)(?::(\d+).*|–\d+.*)?)?$"#).unwrap();
    static ref TWEAK_1: Regex = Regex::new("^([123]|Song) ").unwrap();
    static ref TWEAK_2: Regex = Regex::new("([a-z]) ([0-9]+)$").unwrap();
}

/// I use non-breaking spaces in suitable places in Bible references.
///
/// Here are the rules I’m using (with underscore representing non-breaking space, for convenience):
///
/// 1. In numbered books, the number MUST stay with next: “1 John” → “1_John”.
///
/// 2. “Song of Solomon” becomes “Song_of Solomon” (it’s subjectively a touch nicer).
///
/// 3. “verse N” MUST stay together as “verse_N”.
///
/// 4. If book and chapter only, chapter must stay with previous: “Book N” → “Book_N”.
///
/// In all other circumstances, breaking spaces are retained.
fn tweak_bible_ref_spacing(display: &str) -> String {
    let display = TWEAK_1.replace_all(display, "$1\u{a0}");
    let display = TWEAK_2.replace_all(&display, "$1\u{a0}$2");
    display.into()
}

macro_rules! bible_ref_or_refs_filter {
    ($name:ident, $regex:ident) => {
        pub fn $name<S: BuildHasher>(
            value: &Value,
            args: &HashMap<String, Value, S>,
        ) -> TeraResult<Value> {
            let link = match args.get("link") {
                Some(val) => try_get_value!(stringify!($name), "link", bool, val),
                None => true,
            };
            let s = try_get_value!(stringify!($name), "value", String, value);
            let mut error = Ok(());
            let out = $regex.replace_all(&s, |captures: &Captures| {
                match bible_ref_replacer(stringify!($name), captures, link) {
                    Ok(s) => s,
                    Err(e) => {
                        error = Err(e);
                        String::new()
                    }
                }
            });
            error?;
            Ok(to_value(out).unwrap())
        }
    }
}

bible_ref_or_refs_filter!(bible_refs, BIBLE_REFS);
bible_ref_or_refs_filter!(bible_ref, BIBLE_REF);

fn bible_ref_replacer(filter: &str, captures: &Captures, link: bool) -> TeraResult<String> {
    let display = &captures[1];
    let reference = captures.get(2).map(|m| m.as_str()).unwrap_or(display);
    let add_title_attribute = reference != display;

    // Now we turn it into something suitable for ebible.org links.
    let reference_captures = match REFERENCE_PARSER.captures(reference) {
        Some(c) => c,
        None => return Err(TeraError::msg(format!(
            "Filter `{}` couldn’t parse the reference `{}` (in `{}`)",
            filter,
            reference,
            captures.get(0).unwrap().as_str(),
        ))),
    };
    let book = &reference_captures[1];
    let s2 = reference_captures.get(2).map(|m| m.as_str());
    let s3 = reference_captures.get(3).map(|m| m.as_str());
    let (chapter, verse_number) = match book {
        "Obadiah" | "Philemon" | "2 John" | "3 John" | "Jude" => {
            if s3.is_some() {
                // Yeah, I’m not even accepting “Obadiah 1:3” or such.
                return Err(TeraError::msg(format!(
                    "{} has one chapter only; omit the chapter number (filter `{}` couldn’t cope in `{}`)",
                    book,
                    filter,
                    captures.get(0).unwrap().as_str(),
                )));
            }
            ("1", s2)
        },
        _ => (s2.unwrap_or("1"), s3),
    };
    let ebible_book = match book {
        "Genesis" => "GEN",
        "Exodus" => "EXO",
        "Leviticus" => "LEV",
        "Numbers" => "NUM",
        "Deuteronomy" => "DEU",
        "Joshua" => "JOS",
        "Judges" => "JDG",
        "Ruth" => "RUT",
        "1 Samuel" => "1SA",
        "2 Samuel" => "2SA",
        "1 Kings" => "1KI",
        "2 Kings" => "2KI",
        "1 Chronicles" => "1CH",
        "2 Chronicles" => "2CH",
        "Ezra" => "EZR",
        "Nehemiah" => "NEH",
        "Esther" => "EST",
        "Job" => "JOB",
        "Psalm" | "Psalms" if chapter.len() < 3 => "PSA0",
        "Psalm" | "Psalms" => "PSA",
        "Proverbs" => "PRO",
        "Ecclesiastes" => "ECC",
        "Song of Solomon" => "SNG",
        "Isaiah" => "ISA",
        "Jeremiah" => "JER",
        "Lamentations" => "LAM",
        "Ezekiel" => "EZK",
        "Daniel" => "DAN",
        "Hosea" => "HOS",
        "Joel" => "JOL",
        "Amos" => "AMO",
        "Obadiah" => "OBA",
        "Jonah" => "JON",
        "Micah" => "MIC",
        "Nahum" => "NAM",
        "Habakkuk" => "HAB",
        "Zephaniah" => "ZEP",
        "Haggai" => "HAG",
        "Zechariah" => "ZEC",
        "Malachi" => "MAL",
        "Matthew" => "MAT",
        "Mark" => "MRK",
        "Luke" => "LUK",
        "John" => "JHN",
        "Acts" => "ACT",
        "Romans" => "ROM",
        "1 Corinthians" => "1CO",
        "2 Corinthians" => "2CO",
        "Galatians" => "GAL",
        "Ephesians" => "EPH",
        "Philippians" => "PHP",
        "Colossians" => "COL",
        "1 Thessalonians" => "1TH",
        "2 Thessalonians" => "2TH",
        "1 Timothy" => "1TI",
        "2 Timothy" => "2TI",
        "Titus" => "TIT",
        "Philemon" => "PHM",
        "Hebrews" => "HEB",
        "James" => "JAS",
        "1 Peter" => "1PE",
        "2 Peter" => "2PE",
        "1 John" => "1JN",
        "2 John" => "2JN",
        "3 John" => "3JN",
        "Jude" => "JUD",
        "Revelation" => "REV",
        _ => return Err(TeraError::msg(format!(
            "Filter `{}` couldn’t cope with the reference `{}` (in `{}`; detected book name `{}`, but that’s not a known book name)",
            filter,
            reference,
            captures.get(0).unwrap().as_str(),
            book,
        ))),
    };
    let (title1, title2, title3) = if add_title_attribute {
        (" title=\"", reference, "\"")
    } else {
        ("", "", "")
    };
    if link {
        Ok(format!(r#"<a class=bible-reference href="https://ebible.org/engwebpb/{}{}{}.htm{}{}"{}{}{}>{}</a>"#,
            ebible_book,
            if chapter.len() == 1 { "0" } else { "" },
            chapter,
            if verse_number.is_some() { "#V" } else { "" },
            if let Some(num) = verse_number { num } else { "" },
            title1, title2, title3,
            tweak_bible_ref_spacing(display),
        ))
    } else {
        Ok(format!(r#"<span class=bible-reference{}{}{}>{}</span>"#,
            title1, title2, title3,
            tweak_bible_ref_spacing(display),
        ))
    }
}

pub fn markdown<S: BuildHasher>(
    value: &Value,
    args: &HashMap<String, Value, S>,
) -> TeraResult<Value> {
    let s = try_get_value!("markdown", "value", String, value);
    let inline = match args.get("inline") {
        Some(val) => try_get_value!("markdown", "inline", bool, val),
        None => false,
    };

    let mut opts = cmark::Options::empty();
    opts.insert(cmark::Options::ENABLE_TABLES);
    opts.insert(cmark::Options::ENABLE_FOOTNOTES);
    opts.insert(cmark::Options::ENABLE_STRIKETHROUGH);

    let mut html = String::new();
    let parser = cmark::Parser::new_ext(&s, opts);
    cmark::html::push_html(&mut html, parser);

    if inline {
        html = html
            .trim_start_matches("<p>")
            // pulldown_cmark finishes a paragraph with `</p>\n`
            .trim_end_matches("</p>\n")
            .to_string();
    }

    Ok(to_value(&html).unwrap())
}

pub fn base64_encode<S: BuildHasher>(
    value: &Value,
    _: &HashMap<String, Value, S>,
) -> TeraResult<Value> {
    let s = try_get_value!("base64_encode", "value", String, value);
    Ok(to_value(&encode(s.as_bytes())).unwrap())
}

pub fn base64_decode<S: BuildHasher>(
    value: &Value,
    _: &HashMap<String, Value, S>,
) -> TeraResult<Value> {
    let s = try_get_value!("base64_decode", "value", String, value);
    Ok(to_value(&String::from_utf8(decode(s.as_bytes()).unwrap()).unwrap()).unwrap())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tera::to_value;

    use super::{base64_decode, base64_encode, markdown, replace_all, bible_refs};

    #[test]
    fn bible_refs_filter() {
        let args = &HashMap::new();
        macro_rules! case {
            ($src:expr, $expected:expr) => {
                let src = $src;
                let result = bible_refs(&to_value(src).unwrap(), &args);
                if !result.is_ok() {
                    panic!("Result of bible_refs filter on {:?} was not OK: {:?}", src, result);
                }
                assert_eq!(result.unwrap(), to_value($expected).unwrap());
            };
        }
        case!("In ★Genesis 1:1★, God created.",
            r#"In <a class=bible-reference href="https://ebible.org/engwebpb/GEN01.htm#V1">Genesis 1:1</a>, God created."#);
        case!("In ★verse 1 &lt;Genesis 1:1&gt;★, God created.",
            r#"In <a class=bible-reference href="https://ebible.org/engwebpb/GEN01.htm#V1" title="Genesis 1:1">verse 1</a>, God created."#);
        case!("I picked ★Song of Solomon 2:17★ randomly.",
            r#"I picked <a class=bible-reference href="https://ebible.org/engwebpb/SNG02.htm#V17">Song of Solomon 2:17</a> randomly."#);
        case!("How about ★chapter 4 &lt;Song of Solomon 4&gt;★ or ★Judges 6★?",
            r#"How about <a class=bible-reference href="https://ebible.org/engwebpb/SNG04.htm" title="Song of Solomon 4">chapter 4</a> or <a class=bible-reference href="https://ebible.org/engwebpb/JDG06.htm">Judges 6</a>?"#);
        case!("What of psalms with such diverse numbers? ★Psalms 3 &lt;Psalm 3&gt;★, ★15:2 <Psalm 15:2&gt;★, ★119:174–176 &lt;Psalm 119:174–176>★!",
            r#"What of psalms with such diverse numbers? <a class=bible-reference href="https://ebible.org/engwebpb/PSA003.htm" title="Psalm 3">Psalms 3</a>, <a class=bible-reference href="https://ebible.org/engwebpb/PSA015.htm#V2" title="Psalm 15:2">15:2</a>, <a class=bible-reference href="https://ebible.org/engwebpb/PSA119.htm#V174" title="Psalm 119:174–176">119:174–176</a>!"#);
        assert!(bible_refs(&to_value("★Hezekiah 3:16★ is not a thing.").unwrap(), &args).is_err());
        assert!(bible_refs(&to_value("★Obadiah 1:12★ isn’t allowed.").unwrap(), &args).is_err());
        case!("Express it as ★Obadiah 12★",
            r#"Express it as <a class=bible-reference href="https://ebible.org/engwebpb/OBA01.htm#V12">Obadiah 12</a>"#);
        case!("Though ★Obadiah 1:12 <Obadiah 12>★ would work too! 😛",
            r#"Though <a class=bible-reference href="https://ebible.org/engwebpb/OBA01.htm#V12" title="Obadiah 12">Obadiah 1:12</a> would work too! 😛"#);
        case!("One last major case, numbered books: ★1 John 5★, ★2 John 4★, ★1 John 3:12–15, 17★",
            r#"One last major case, numbered books: <a class=bible-reference href="https://ebible.org/engwebpb/1JN05.htm">1 John 5</a>, <a class=bible-reference href="https://ebible.org/engwebpb/2JN01.htm#V4">2 John 4</a>, <a class=bible-reference href="https://ebible.org/engwebpb/1JN03.htm#V12">1 John 3:12–15, 17</a>"#);
        // TODO: test the bible_ref filter and the link parameter.
    }

    #[test]
    fn replace_all_filter() {
        let mut args = HashMap::new();
        args.insert("regex".to_string(), to_value("(\\d{4})-(\\d{2})-(\\d{2})").unwrap());
        args.insert("rep".to_string(), to_value("$3/$2/$1").unwrap());
        let result = replace_all(
            &to_value(&"The date is 1234-56-78. Or is it 2345-67-89?").unwrap(),  // No.
            &args,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), to_value(&"The date is 78/56/1234. Or is it 89/67/2345?").unwrap());
    }

    #[test]
    fn markdown_filter() {
        let result = markdown(&to_value(&"# Hey").unwrap(), &HashMap::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), to_value(&"<h1>Hey</h1>\n").unwrap());
    }

    #[test]
    fn markdown_filter_inline() {
        let mut args = HashMap::new();
        args.insert("inline".to_string(), to_value(true).unwrap());
        let result = markdown(
            &to_value(&"Using `map`, `filter`, and `fold` instead of `for`").unwrap(),
            &args,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), to_value(&"Using <code>map</code>, <code>filter</code>, and <code>fold</code> instead of <code>for</code>").unwrap());
    }

    // https://github.com/Keats/gutenberg/issues/417
    #[test]
    fn markdown_filter_inline_tables() {
        let mut args = HashMap::new();
        args.insert("inline".to_string(), to_value(true).unwrap());
        let result = markdown(
            &to_value(
                &r#"
|id|author_id|       timestamp_created|title                 |content           |
|-:|--------:|-----------------------:|:---------------------|:-----------------|
| 1|        1|2018-09-05 08:03:43.141Z|How to train your ORM |Badly written blog|
| 2|        1|2018-08-22 13:11:50.050Z|How to bake a nice pie|Badly written blog|
        "#,
            )
            .unwrap(),
            &args,
        );
        assert!(result.is_ok());
        assert!(result.unwrap().as_str().unwrap().contains("<table>"));
    }

    #[test]
    fn base64_encode_filter() {
        // from https://tools.ietf.org/html/rfc4648#section-10
        let tests = vec![
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ];
        for (input, expected) in tests {
            let args = HashMap::new();
            let result = base64_encode(&to_value(input).unwrap(), &args);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), to_value(expected).unwrap());
        }
    }

    #[test]
    fn base64_decode_filter() {
        let tests = vec![
            ("", ""),
            ("Zg==", "f"),
            ("Zm8=", "fo"),
            ("Zm9v", "foo"),
            ("Zm9vYg==", "foob"),
            ("Zm9vYmE=", "fooba"),
            ("Zm9vYmFy", "foobar"),
        ];
        for (input, expected) in tests {
            let args = HashMap::new();
            let result = base64_decode(&to_value(input).unwrap(), &args);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), to_value(expected).unwrap());
        }
    }
}
