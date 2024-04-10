use crud_pretty_struct::formatters::bool_check_formatter;
use crud_pretty_struct::PrettyPrint;

#[test]
fn empty_struct() {
  #[derive(PrettyPrint)]
  struct T1 {}

  let s = T1 {};
  assert_eq!(s.pretty(false, None, None).unwrap(), "".to_string());
}

#[test]
fn simple_struct() {
  #[derive(PrettyPrint)]
  struct T2 {
    a: u32,
    bb: String,
    cccc: bool,
  }

  let s = T2 {
    a: 5,
    bb: "string".to_string(),
    cccc: false,
  };
  //  print!("{}", s.pretty(false, None, None));
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a    = 5\nbb   = string\ncccc = false\n".to_string()
  );
}

#[test]
fn profile_struct() {
  #[derive(PrettyPrint)]
  struct T2 {
    #[pretty(profiles = "a")]
    a: u32,
    #[pretty(profiles = "b")]
    bb: String,
    #[pretty(profiles = "a,b")]
    cccc: bool,
  }

  let s = T2 {
    a: 5,
    bb: "string".to_string(),
    cccc: false,
  };
  //  print!("{}", s.pretty(false, None, None));
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a    = 5\nbb   = string\ncccc = false\n".to_string()
  );
  assert_eq!(
    s.pretty(false, None, Some("a")).unwrap(),
    "a    = 5\ncccc = false\n".to_string()
  );
  assert_eq!(
    s.pretty(false, None, Some("b")).unwrap(),
    "bb   = string\ncccc = false\n".to_string()
  );
}

#[test]
fn label_struct() {
  #[derive(PrettyPrint)]
  struct T2 {
    #[pretty(label = "label a")]
    a: u32,
    #[pretty(label = "label b")]
    bb: String,
    #[pretty(label = "label c")]
    cccc: bool,
  }

  let s = T2 {
    a: 5,
    bb: "string".to_string(),
    cccc: false,
  };
  //  print!("{}", s.pretty(false, None, None));
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "label a = 5\nlabel b = string\nlabel c = false\n".to_string()
  );
}

#[test]
fn simple_struct_custom_separator() {
  #[derive(PrettyPrint)]
  #[pretty(separator_glyph = "-> ")]
  struct T2 {
    a: u32,
    bb: String,
    cccc: bool,
  }

  let s = T2 {
    a: 5,
    bb: "string".to_string(),
    cccc: false,
  };
  //  print!("{}", s.pretty(false, None, None));
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a    -> 5\nbb   -> string\ncccc -> false\n".to_string()
  );
}

#[test]
fn nested_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: u32,
    bb: String,
    cccc: bool,
  }
  #[derive(PrettyPrint)]
  struct T2 {
    a: u32,
    #[pretty(is_pretty)]
    n: T1,
  }

  let s = T2 {
    a: 5,
    n: T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    },
  };
  //  print!("{}", s.pretty(false, None, None));
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 5\nn -->\n| a    = 5\n| bb   = string\n| cccc = false\n".to_string()
  );
}

#[test]
fn skip_none_option_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    #[pretty(skip_none)]
    a: Option<u32>,
    bb: Option<String>,
  }

  let s = T1 { a: None, bb: None };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "bb = null\n".to_string()
  );
}

#[test]
fn simple_struct_colored() {
  #[derive(PrettyPrint)]
  struct T2 {
    a: u32,
    bb: String,
    cccc: bool,
  }

  let s = T2 {
    a: 5,
    bb: "string".to_string(),
    cccc: false,
  };
  //  print!("{}", s.pretty(true, None));
  assert_eq!(
    s.pretty(true, None, None).unwrap(),
    "a    = \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\nbb   = \u{1b}[1m\u{1b}[97mstring\u{1b}[39m\u{1b}[0m\ncccc = \u{1b}[1m\u{1b}[97mfalse\u{1b}[39m\u{1b}[0m\n".to_string()
  );
}

#[test]
fn simple_struct_custom_color() {
  #[derive(PrettyPrint)]
  struct T2 {
    #[pretty(color = "red")]
    a: u32,
    #[pretty(color = "green")]
    bb: String,
    #[pretty(color = "cyan")]
    cccc: bool,
  }

  let s = T2 {
    a: 5,
    bb: "string".to_string(),
    cccc: false,
  };
  //  print!("{}", s.pretty(true, None, None));
  assert_eq!(
    s.pretty(true, None, None).unwrap(),
      "a    = \u{1b}[1m\u{1b}[31m5\u{1b}[39m\u{1b}[0m\nbb   = \u{1b}[1m\u{1b}[32mstring\u{1b}[39m\u{1b}[0m\ncccc = \u{1b}[1m\u{1b}[36mfalse\u{1b}[39m\u{1b}[0m\n".to_string()
  );
}

#[test]
fn simple_struct_custom_label_color() {
  #[derive(PrettyPrint)]
  struct T2 {
    #[pretty(label_color = "red")]
    a: u32,
    #[pretty(label_color = "green")]
    bb: String,
    #[pretty(label_color = "cyan")]
    cccc: bool,
  }

  let s = T2 {
    a: 5,
    bb: "string".to_string(),
    cccc: false,
  };
  //  print!("{}", s.pretty(true, None, None));
  assert_eq!(
    s.pretty(true, None, None).unwrap(),
     "\u{1b}[31ma\u{1b}[39m= \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\n\u{1b}[32mbb\u{1b}[39m= \u{1b}[1m\u{1b}[97mstring\u{1b}[39m\u{1b}[0m\n\u{1b}[36mcccc\u{1b}[39m= \u{1b}[1m\u{1b}[97mfalse\u{1b}[39m\u{1b}[0m\n".to_string()
  );
}

#[test]
fn skip_simple_struct() {
  #[derive(PrettyPrint)]
  #[allow(dead_code)]
  struct T2 {
    a: u32,
    #[pretty(skip)]
    bb: String,
    #[pretty(skip)]
    cccc: bool,
  }

  let s = T2 {
    a: 5,
    bb: "string".to_string(),
    cccc: false,
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(s.pretty(false, None, None).unwrap(), "a = 5\n".to_string());
}

#[test]
fn skip_nested_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: u32,
    bb: String,
    cccc: bool,
  }
  #[derive(PrettyPrint)]
  #[allow(dead_code)]
  struct T2 {
    a: u32,
    #[pretty(is_pretty, skip)]
    n: T1,
  }

  let s = T2 {
    a: 5,
    n: T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    },
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(s.pretty(false, None, None).unwrap(), "a = 5\n".to_string());
}

#[test]
fn option_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: Option<u32>,
    bb: Option<String>,
  }

  let s = T1 {
    a: Some(5),
    bb: None,
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a  = 5\nbb = null\n".to_string()
  );
}

#[test]
fn formatter_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    #[pretty(formatter = |x,_|Ok((format!("{} format",x.to_string()),false)))]
    a: u32,
    #[pretty(formatter = bool_check_formatter)]
    s: bool,
    #[pretty(formatter = crud_pretty_struct::formatters::bool_check_formatter)]
    bb: Option<String>,
  }

  let s = T1 {
    a: 5,
    s: true,
    bb: Some("false".to_string()),
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a  = 5 format\ns  = ✔\nbb = ✘\n".to_string()
  );
}

#[test]
fn option_struct_colored() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: Option<u32>,
    bb: Option<String>,
  }

  let s = T1 {
    a: Some(5),
    bb: None,
  };
  //  print!("{}", s.pretty(true, None, None));
  assert_eq!(
    s.pretty(true, None, None).unwrap(),
    "a  = \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\nbb = \u{1b}[35mnull\u{1b}[39m\n".to_string()
  );
}

#[test]
fn nested_option_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: u32,
    bb: String,
    cccc: bool,
  }
  #[derive(PrettyPrint)]
  struct T2 {
    a: u32,
    #[pretty(is_pretty)]
    n: Option<T1>,
  }
  let s = T2 {
    a: 5,
    n: Some(T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    }),
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 5\nn -->\n| a    = 5\n| bb   = string\n| cccc = false\n".to_string()
  );

  let s = T2 { a: 5, n: None };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 5\nn = null\n".to_string()
  );
}

#[test]
fn skip_none_nested_option_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: u32,
    bb: String,
    cccc: bool,
  }
  #[derive(PrettyPrint)]
  struct T2 {
    a: u32,
    #[pretty(is_pretty, skip_none)]
    n: Option<T1>,
  }
  let s = T2 {
    a: 5,
    n: Some(T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    }),
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 5\nn -->\n| a    = 5\n| bb   = string\n| cccc = false\n".to_string()
  );

  let s = T2 { a: 5, n: None };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(s.pretty(false, None, None).unwrap(), "a = 5\n".to_string());
}

#[test]
fn vec_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: Vec<u32>,
  }

  let s = T1 {
    a: vec![5, 3, 7, 5],
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a :\n - 5\n - 3\n - 7\n - 5\n".to_string()
  );
}

#[test]
fn vec_struct_colored() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: Vec<u32>,
  }

  let s = T1 {
    a: vec![5, 3, 7, 5],
  };
  //  print!("{}", s.pretty(true, None, None).unwrap());
  assert_eq!(
    s.pretty(true, None, None).unwrap(),
 "a :\n - \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m3\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m7\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\n".to_string()
  );
}

#[test]
fn vec_struct_nested() {
  #[derive(PrettyPrint)]
  struct T1 {
    #[pretty(is_pretty)]
    a: Vec<T2>,
  }

  #[derive(PrettyPrint)]
  struct T2 {
    a: u32,
    bb: String,
    cccc: bool,
  }

  let s = T1 {
    a: vec![
      T2 {
        a: 5,
        bb: "string".to_string(),
        cccc: false,
      },
      T2 {
        a: 5,
        bb: "string".to_string(),
        cccc: false,
      },
    ],
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
 "a :\n - a    = 5\n   bb   = string\n   cccc = false\n - a    = 5\n   bb   = string\n   cccc = false\n"  .to_string()
  );
}

#[test]
fn option_vec_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: Option<Vec<u32>>,
    bb: Option<Vec<String>>,
  }

  let s = T1 {
    a: Some(vec![5, 3, 1, 4, 2]),
    bb: Some(vec!["a".to_string(), "string".to_string()]),
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a :\n - 5\n - 3\n - 1\n - 4\n - 2\nbb :\n - a\n - string\n".to_string()
  );

  let s = T1 { a: None, bb: None };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a : null\nbb : null\n".to_string()
  );
}

#[test]
fn skip_none_option_vec_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    #[pretty(skip_none)]
    a: Option<Vec<u32>>,
    bb: Option<Vec<String>>,
  }

  let s = T1 {
    a: Some(vec![5, 3, 1, 4, 2]),
    bb: Some(vec!["a".to_string(), "string".to_string()]),
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a :\n - 5\n - 3\n - 1\n - 4\n - 2\nbb :\n - a\n - string\n".to_string()
  );

  let s = T1 { a: None, bb: None };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "bb : null\n".to_string()
  );
}

#[test]
fn option_vec_struct_colored() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: Option<Vec<u32>>,
    bb: Option<Vec<String>>,
  }

  let s = T1 {
    a: Some(vec![5, 3, 1, 4, 2]),
    bb: Some(vec!["a".to_string(), "string".to_string()]),
  };
  //  print!("{}", s.pretty(true, None, None).unwrap());
  assert_eq!(
      s.pretty(true, None, None).unwrap(),
      "a :\n - \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m3\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m1\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m4\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97m2\u{1b}[39m\u{1b}[0m\nbb :\n - \u{1b}[1m\u{1b}[97ma\u{1b}[39m\u{1b}[0m\n - \u{1b}[1m\u{1b}[97mstring\u{1b}[39m\u{1b}[0m\n".to_string()
    );

  let s = T1 { a: None, bb: None };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a : null\nbb : null\n".to_string()
  );
}

#[test]
fn nested_option_vec_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: u32,
    bb: String,
    cccc: bool,
  }
  #[derive(PrettyPrint)]
  struct T2 {
    a: u32,
    #[pretty(is_pretty)]
    n: Option<Vec<T1>>,
  }
  let s = T2 {
    a: 5,
    n: Some(vec![T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    }]),
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 5\nn :\n - a    = 5\n   bb   = string\n   cccc = false\n".to_string()
  );

  let s = T2 { a: 5, n: None };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 5\nn : null\n".to_string()
  );

  let s = T2 { a: 5, n: None };
  //  print!("{}", s.pretty(true, None, None).unwrap());
  assert_eq!(
    s.pretty(true, None, None).unwrap(),
    "a = \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\nn :\u{1b}[35m null\n\u{1b}[39m".to_string()
  );
}

#[test]
fn skip_none_nested_option_vec_struct() {
  #[derive(PrettyPrint)]
  struct T1 {
    a: u32,
    bb: String,
    cccc: bool,
  }
  #[derive(PrettyPrint)]
  struct T2 {
    a: u32,
    #[pretty(is_pretty, skip_none)]
    n: Option<Vec<T1>>,
  }
  let s = T2 {
    a: 5,
    n: Some(vec![T1 {
      a: 5,
      bb: "string".to_string(),
      cccc: false,
    }]),
  };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 5\nn :\n - a    = 5\n   bb   = string\n   cccc = false\n".to_string()
  );

  let s = T2 { a: 5, n: None };
  //  print!("{}", s.pretty(false, None, None).unwrap());
  assert_eq!(s.pretty(false, None, None).unwrap(), "a = 5\n".to_string());

  let s = T2 { a: 5, n: None };
  //  print!("{}", s.pretty(true, None, None));
  assert_eq!(
    s.pretty(true, None, None).unwrap(),
    "a = \u{1b}[1m\u{1b}[97m5\u{1b}[39m\u{1b}[0m\n".to_string()
  );
}

#[test]
fn simple_enum() {
  #[derive(PrettyPrint)]
  enum E {
    AA,
    BB,
  }

  let s = E::AA;
  assert_eq!(s.pretty(false, None, None).unwrap(), "AA\n".to_string());

  let s = E::BB;
  assert_eq!(s.pretty(false, None, None).unwrap(), "BB\n".to_string());
}

#[test]
fn simple_enum_in_struct() {
  #[derive(PrettyPrint)]
  enum E {
    AA,
    BB,
  }

  #[derive(PrettyPrint)]
  struct S {
    a: u32,
    #[pretty(is_pretty)]
    b: E,
    #[pretty(is_pretty)]
    bb: E,
    c: u32,
  }

  let s = S {
    a: 1,
    b: E::AA,
    bb: E::BB,
    c: 3,
  };

  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a  = 1\nb  = AA\nbb = BB\nc  = 3\n".to_string()
  );
}

#[test]
fn vec_enum_in_struct() {
  #[derive(PrettyPrint)]
  enum E {
    AA,
    BB,
  }

  #[derive(PrettyPrint)]
  struct S {
    a: u32,
    #[pretty(is_pretty)]
    b: Vec<E>,
    c: u32,
  }

  let s = S {
    a: 1,
    b: vec![E::AA, E::BB],
    c: 3,
  };

  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 1\nb :\n - AA\n - BB\nc = 3\n".to_string()
  );
}

#[test]
fn option_vec_enum_in_struct() {
  #[derive(PrettyPrint)]
  enum E {
    AA,
    BB,
  }

  #[derive(PrettyPrint)]
  struct S {
    a: u32,
    #[pretty(is_pretty)]
    b: Option<Vec<E>>,
    c: u32,
  }

  let s = S {
    a: 1,
    b: Some(vec![E::AA, E::BB]),
    c: 3,
  };

  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 1\nb :\n - AA\n - BB\nc = 3\n".to_string()
  );

  let s = S {
    a: 1,
    b: None,
    c: 3,
  };

  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "a = 1\nb : null\nc = 3\n".to_string()
  );
}

#[test]
fn tuple_enum() {
  #[derive(PrettyPrint)]
  struct St {
    aa: u32,
    cc: u32,
  }

  #[derive(PrettyPrint)]
  enum E {
    AA(St),
  }

  let s = E::AA(St { aa: 2, cc: 4 });
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "aa = 2\ncc = 4\n".to_string()
  );
}

#[test]
fn enum_with_label() {
  #[derive(PrettyPrint)]
  enum E {
    AA,
    #[pretty(label = "My Label")]
    BB,
  }

  let s = E::AA;
  assert_eq!(s.pretty(false, None, None).unwrap(), "AA\n".to_string());

  let s = E::BB;
  assert_eq!(
    s.pretty(false, None, None).unwrap(),
    "My Label\n".to_string()
  );
}
