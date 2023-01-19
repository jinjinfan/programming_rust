fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut upper = 's'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);

    let spacey = "man hat tan";
    let spaceless : String = spacey.chars().filter(|c|!c.is_whitespace()).collect();
    assert_eq!(spaceless, "manhattan");

    let mut choco = "chocolate".to_string();
    assert_eq!(choco.drain(3..6).collect::<String>(), "col");
    assert_eq!(choco, "choate");

    assert_eq!("## Elephants"
                .trim_start_matches(|ch: char| ch == '#' || ch.is_whitespace()),
                "Elephants");
    
    let code = "\t    function noodle() {";
    assert_eq!(code.trim_start_matches([' ', '\t'].as_ref()),
        "function noodle() {"
    );
    assert_eq!("cabababababbage".replace("aba", "***"), "c***b***babbage");
    assert_eq!("alan".char_indices().collect::<Vec<_>>(),
                vec![
                    (0, 'a'),
                    (1, 'l'),
                    (2, 'a'),
                    (3, 'n'),
                ]);
    
    assert_eq!("jimb:1000:Jim Blandy:".split(":").collect::<Vec<_>>(),
                vec!["jimb", "1000", "Jim Blandy", ""]);
    assert_eq!("127.0.0.1 localhost\n\
                127.0.0.1 www.reddit.com\n"
                .split_terminator('\n').collect::<Vec<_>>(),
              vec!["127.0.0.1 localhost", "127.0.0.1 www.reddit.com"]);

    let slice = "banana";
    assert_eq!(slice.strip_suffix("na"), Some("bana"));

    let bad_utf8 : Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
    let result = String::from_utf8(bad_utf8);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().into_bytes(),
        vec![0x9f, 0xf0, 0xa6, 0x80]);
    
    use std::borrow::Cow;
    fn get_name() -> Cow<'static, str> {
        std::env::var("USER")
            .map(|v| Cow::Owned(v))
            .unwrap_or(Cow::Borrowed("whoever you are"))
    }
    fn get_name2()-> Cow<'static, str> {
        std::env::var("USER")
            .map(|v| v.into())
            .unwrap_or(Cow::Borrowed("whoever you are"))
    }

    fn get_title() -> Option<&'static str> {
        Some("Esq.")
    }
    let mut name = get_name();
    if let Some(title) = get_title() {
        name.to_mut().push_str(", ");
        name += title;
    }
    println!("Greeting, {}", name);

    use std::rc::Rc;
    let original = Rc::new("hello".to_string());
    let cloned = original.clone();
    let imposter = Rc::new("hello".to_string());
    println!("text:    {}, {}, {}", original, cloned, imposter);
    println!("pointers:{:p}, {:p}, {:p}", original, cloned, imposter);

    assert_eq!(format!("{mode} {2} {} {}", "people", "eater", "purple", mode="flying"),
               "flying purple people eater");
    // Dynamic widths and precisions
    //format!("{:>1$}",content, get_width())
    //format!("{:>width?}",content, width=get_width())
    //format!("{:.*}", get_limit(), content)

    struct Complex {
        re : f64,
        im : f64
    }
    use std::fmt;
    impl fmt::Display for Complex {
        fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
            let (re, im) = (self.re, self.im);
            if dest.alternate() {
                let abs = f64::sqrt(re*re+im*im);
                let angle = f64::atan2(im, re)/std::f64::consts::PI * 180.0;
                write!(dest, "{} < {}.", abs, angle)
            } else {
                let im_sign = if self.im < 0.0 {'-'} else {'+'};
                write!(dest, "{} {} {}i", self.re, im_sign, f64::abs(self.im))
            }
        }
    }
    let one_twenty = Complex{re: -0.5, im : 0.866};
    assert_eq!(format!("{}", one_twenty), "-0.5 + 0.866i");
    println!("{:#}", one_twenty);
    let two_forty = Complex{re: -0.5, im : -0.866};
    assert_eq!(format!("{}", one_twenty), "-0.5 + 0.866i");
    println!("{:#}", two_forty);

    fn logging_enabled() -> bool {
        true
    }
    use std::fs::OpenOptions;
    use std::io::Write;

    fn write_log_entry(entry: std::fmt::Arguments) {
        if logging_enabled() {
            let mut log_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("log-file-name")
                .expect("fail to open log file");
            log_file.write_fmt(entry).expect("fail to write to log");
        }
    }
    #[derive(Debug)]
    struct Complex2 {
        re : f64,
        im : f64
    }
    let value = Complex2{re: -0.5, im : 0.866};
    write_log_entry(format_args!("Hark! {:?}\n", value));
    
    macro_rules! log {
        ($format:tt, $($arg:expr),*) => {
            write_log_entry(format_args!($format,$($arg),*))
        };
    }
    log!("0 day and night, but {:?}\n", value);
    use regex::Regex;
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?")?;
    let haystack = r#"regex = "0.2.5""#;
    assert!(semver.is_match(haystack));
    let captures = semver.captures(haystack).ok_or("semver regex should have matched")?;
    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");

    assert_eq!(captures.get(4), None);
    assert_eq!(captures.get(3).unwrap().start(), 13);
    assert_eq!(captures.get(3).unwrap().end(), 14);
    assert_eq!(captures.get(3).unwrap().as_str(), "5");

    let test = "In the beginning, there was 1.0.0. \
                For a while, we used 1.0.1-beta, \
                but in the end, we settled on 1.2.4.";
    let matches: Vec<&str> = semver.find_iter(test)
        .map(|match_| match_.as_str())
        .collect();
    assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);

    use lazy_static::lazy_static;

    lazy_static! {
        static ref SEMVER: Regex = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").expect("error parsing regex");
    }
    if let Some(match_) = SEMVER.find(&test) {
        println!("{}", match_.as_str());
    }

    use unicode_normalization::UnicodeNormalization;
    

    Ok(())
}
