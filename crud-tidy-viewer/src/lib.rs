//! # Crud tidy viewer
//!
//! Pretty printer for arrays.
//!
//! Some parts of the code is copied and refactored from [Tidy-viewer](https://github.com/alexhallam/tv) (released under public domain)
//!
//! ## Examples
//!
//! ```rust
//! use crud_tidy_viewer::{display_table, TableConfig};
//! # use miette::Result;
//! # fn main() -> Result<()> {
//!   let rdr = vec![
//!     vec!["a".to_string(), "b".to_string()],
//!     vec!["1".to_string(), "b".to_string()],
//!     vec!["4.1453".to_string(), "c".to_string()],
//!     vec!["2.4".to_string(), "f".to_string()],
//!     vec!["5".to_string(), "e".to_string()],
//!   ];
//!   display_table(&rdr, TableConfig::default());
//! #  Ok(())
//! # }
//! ```

mod datatype;
use calm_io::stdout;
use calm_io::stdoutln;
use owo_colors::OwoColorize;

pub struct TableConfig {
  std_color: [u8; 3],
  neg_num_color: [u8; 3],
  na_color: [u8; 3],
  meta_color: [u8; 3],
  header_color: [u8; 3],
  title_option: String,
  footer_option: String,
  display_meta: bool,
  extend_option: bool,
  line_counter: bool,
  debug_mode: bool,
  is_tty: bool,
  is_force_color: bool,
  term_tuple: (u16, u16),
  sigfig: i64,
  lower_column_width: usize,
  upper_column_width: usize,
  row_display_option: usize,
}

const NORD_META_COLOR: [u8; 3] = [143, 188, 187];
const NORD_HEADER_COLOR: [u8; 3] = [94, 129, 172];
const NORD_STD_COLOR: [u8; 3] = [216, 222, 233];
const NORD_NA_COLOR: [u8; 3] = [191, 97, 106];
const NORD_NEG_NUM_COLOR: [u8; 3] = [208, 135, 112];

impl Default for TableConfig {
  fn default() -> Self {
    Self {
      std_color: NORD_STD_COLOR,
      neg_num_color: NORD_NEG_NUM_COLOR,
      na_color: NORD_NA_COLOR,
      meta_color: NORD_META_COLOR,
      header_color: NORD_HEADER_COLOR,
      title_option: Default::default(),
      footer_option: Default::default(),
      display_meta: false,
      extend_option: true,
      line_counter: false,
      debug_mode: false,
      is_tty: atty::is(atty::Stream::Stdout),
      is_force_color: false,
      term_tuple: crossterm::terminal::size().unwrap(),
      sigfig: 3,
      lower_column_width: 2,
      upper_column_width: 50,
      row_display_option: 25,
    }
  }
}

pub fn display_table(rdr: &[Vec<String>], config: TableConfig) {
  /*
      This piece of code is copied and refactored from Tidy-viewer (released under public domain)
      Original source: https://github.com/alexhallam/tv
      commit: 973d88f7ed05a394b5309c5e8b6f3e3a52a39b17 /  May 14, 2022
  */

  let cols: usize = rdr[0].len();
  let rows_in_file: usize = rdr.len();
  let rows: usize = if config.extend_option {
    rdr.len().min(rows_in_file + 1)
  } else {
    rdr.len().min(config.row_display_option + 1)
  };

  let rows_remaining: usize = rows_in_file - rows;
  let ellipsis = '\u{2026}'.to_string();
  let row_remaining_text: String = format!("{ellipsis} with {rows_remaining} more rows");

  // csv gets records in rows. This makes them cols
  let mut v: Vec<Vec<&str>> = Vec::new(); //vec![vec!["#"; rows as usize]; cols as usize];
  for col in 0..cols {
    let column = rdr
      .iter()
      .take(rows)
      .map(|row| row.get(col).unwrap().as_str())
      .collect();
    v.push(column)
  }

  if config.debug_mode {
    println!("{:?}", "v");
    println!("{v:?}");
  }

  if config.debug_mode {
    // make datatypes vector
    let mut vec_datatypes = Vec::with_capacity(cols);
    for column in &v {
      vec_datatypes.push(datatype::get_col_data_type(column))
    }
    println!("{:?}", "vec_datatypes");
    println!("{vec_datatypes:?}");
  }

  // vector of formatted values
  let vf: Vec<Vec<String>> = v
    .iter()
    .map(|col| {
      datatype::format_strings(
        col,
        config.lower_column_width,
        config.upper_column_width,
        config.sigfig,
      )
    })
    .collect();

  if config.debug_mode {
    println!("{:?}", "Transposed Vector of Elements");
    println!("{v:?}");
    println!("{:?}", "Formatted: Vector of Elements");
    println!("{vf:?}");
  }

  //  println!();
  let mut vp = Vec::new();
  for r in 0..rows {
    let row = vf.iter().map(|col| col[r].to_string()).collect();
    vp.push(row);
  }

  let num_cols_to_print = if config.extend_option {
    cols
  } else {
    get_num_cols_to_print(cols, vp.clone(), config.term_tuple)
  };

  // color
  if config.display_meta {
    let meta_text = "tv dim:";
    let div = "x";
    let _ = match stdout!("{: <6}", "") {
      Ok(_) => Ok(()),
      Err(e) => match e.kind() {
        std::io::ErrorKind::BrokenPipe => Ok(()),
        _ => Err(e),
      },
    };
    if config.is_tty || config.is_force_color {
      let _ = match stdoutln!(
        "{} {} {} {}",
        meta_text.truecolor(
          config.meta_color[0],
          config.meta_color[1],
          config.meta_color[2]
        ),
        (rows_in_file - 1).truecolor(
          config.meta_color[0],
          config.meta_color[1],
          config.meta_color[2]
        ),
        div.truecolor(
          config.meta_color[0],
          config.meta_color[1],
          config.meta_color[2]
        ),
        (cols).truecolor(
          config.meta_color[0],
          config.meta_color[1],
          config.meta_color[2]
        ),
      ) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    } else {
      let _ = match stdoutln!("{} {} {} {}", meta_text, rows_in_file - 1, div, cols) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    }
  }
  // title
  if !datatype::is_na(&config.title_option) {
    let _ = match stdout!("{: <6}", "") {
      Ok(_) => Ok(()),
      Err(e) => match e.kind() {
        std::io::ErrorKind::BrokenPipe => Ok(()),
        _ => Err(e),
      },
    };
    if config.is_tty || config.is_force_color {
      let _ = match stdoutln!(
        "{}",
        config
          .title_option
          .truecolor(
            config.meta_color[0],
            config.meta_color[1],
            config.meta_color[2]
          )
          .underline()
          .bold()
      ) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    } else {
      let _ = match stdoutln!("{}", config.title_option) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    }
  }

  // header
  if config.line_counter {
    let _ = match stdout!("{: <6}", "") {
      Ok(_) => Ok(()),
      Err(e) => match e.kind() {
        std::io::ErrorKind::BrokenPipe => Ok(()),
        _ => Err(e),
      },
    };
  }
  //for col in 0..cols {
  for col in 0..num_cols_to_print {
    let text = vp[0].get(col).unwrap().to_string();
    if config.is_tty || config.is_force_color {
      let _ = match stdout!(
        "{}",
        text
          .truecolor(
            config.header_color[0],
            config.header_color[1],
            config.header_color[2]
          )
          .bold()
      ) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    } else {
      let _ = match stdout!("{}", text) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    }
  }
  //println!();
  // datatypes
  //print!("{: <6}", "");
  //for col in 0..cols{
  //    let add_space = vec_datatypes[col].len() - col_largest_width[col];
  //    let mut owned_string: String = vec_datatypes[col].to_string();
  //    let borrowed_string: &str = &" ".repeat(add_space);
  //    owned_string.push_str(borrowed_string);
  //    print!("{}",owned_string.truecolor(143, 188, 187).bold());
  //}
  let _ = match stdoutln!() {
    Ok(_) => Ok(()),
    Err(e) => match e.kind() {
      std::io::ErrorKind::BrokenPipe => Ok(()),
      _ => Err(e),
    },
  };
  vp.iter()
    .enumerate()
    .take(rows)
    .skip(1)
    .for_each(|(i, row)| {
      if config.line_counter {
        if config.is_tty || config.is_force_color {
          let _ = match stdout!(
            "{: <6}",
            i.truecolor(
              config.meta_color[0],
              config.meta_color[1],
              config.meta_color[2]
            )
          ) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
              std::io::ErrorKind::BrokenPipe => Ok(()),
              _ => Err(e),
            },
          };
        } else {
          let _ = match stdout!("{: <6}", i) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
              std::io::ErrorKind::BrokenPipe => Ok(()),
              _ => Err(e),
            },
          };
        }
      }
      row.iter().take(num_cols_to_print).for_each(|col| {
        if config.is_tty || config.is_force_color {
          let _ = match stdout!(
            "{}",
            if datatype::is_na_string_padded(col) {
              col.truecolor(config.na_color[0], config.na_color[1], config.na_color[2])
            } else if datatype::is_number(col) && datatype::is_negative_number(col) {
              col.truecolor(
                config.neg_num_color[0],
                config.neg_num_color[1],
                config.neg_num_color[2],
              )
            } else {
              col.truecolor(
                config.std_color[0],
                config.std_color[1],
                config.std_color[2],
              )
            }
          ) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
              std::io::ErrorKind::BrokenPipe => Ok(()),
              _ => Err(e),
            },
          };
        } else {
          let _ = match stdout!("{}", col) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
              std::io::ErrorKind::BrokenPipe => Ok(()),
              _ => Err(e),
            },
          };
        }
      });
      let _ = match stdoutln!() {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    });

  // additional row info
  if rows_remaining > 0 {
    let _ = match stdout!("{: <6}", "") {
      Ok(_) => Ok(()),
      Err(e) => match e.kind() {
        std::io::ErrorKind::BrokenPipe => Ok(()),
        _ => Err(e),
      },
    };
    if config.is_tty || config.is_force_color {
      let _ = match stdout!(
        "{}",
        row_remaining_text.truecolor(
          config.meta_color[0],
          config.meta_color[1],
          config.meta_color[2]
        )
      ) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    } else {
      let _ = match stdout!("{}", row_remaining_text) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    }
    let extra_cols_to_mention = num_cols_to_print;
    let remainder_cols = cols - extra_cols_to_mention;
    if extra_cols_to_mention < cols {
      let meta_text_and = "and";
      let meta_text_var = "more variables";
      let meta_text_comma = ",";
      let meta_text_colon = ":";
      if config.is_tty || config.is_force_color {
        let _ = match stdout!(
          " {} {} {}{}",
          meta_text_and.truecolor(
            config.meta_color[0],
            config.meta_color[1],
            config.meta_color[2]
          ),
          remainder_cols.truecolor(
            config.meta_color[0],
            config.meta_color[1],
            config.meta_color[2]
          ),
          meta_text_var.truecolor(
            config.meta_color[0],
            config.meta_color[1],
            config.meta_color[2]
          ),
          meta_text_colon.truecolor(
            config.meta_color[0],
            config.meta_color[1],
            config.meta_color[2]
          )
        ) {
          Ok(_) => Ok(()),
          Err(e) => match e.kind() {
            std::io::ErrorKind::BrokenPipe => Ok(()),
            _ => Err(e),
          },
        };
      } else {
        let _ = match stdout!(
          " {} {} {}{}",
          meta_text_and,
          remainder_cols,
          meta_text_var,
          meta_text_colon
        ) {
          Ok(_) => Ok(()),
          Err(e) => match e.kind() {
            std::io::ErrorKind::BrokenPipe => Ok(()),
            _ => Err(e),
          },
        };
      }
      for col in extra_cols_to_mention..cols {
        let text = rdr[0].get(col).unwrap();
        if config.is_tty || config.is_force_color {
          let _ = match stdout!(
            " {}",
            text.truecolor(
              config.meta_color[0],
              config.meta_color[1],
              config.meta_color[2]
            )
          ) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
              std::io::ErrorKind::BrokenPipe => Ok(()),
              _ => Err(e),
            },
          };
        } else {
          let _ = match stdout!(" {}", text) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
              std::io::ErrorKind::BrokenPipe => Ok(()),
              _ => Err(e),
            },
          };
        }

        // The last column mentioned in foot should not be followed by a comma
        if col + 1 < cols {
          if config.is_tty || config.is_force_color {
            let _ = match stdout!(
              "{}",
              meta_text_comma.truecolor(
                config.meta_color[0],
                config.meta_color[1],
                config.meta_color[2]
              )
            ) {
              Ok(_) => Ok(()),
              Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
              },
            };
          } else {
            let _ = match stdout!("{}", meta_text_comma) {
              Ok(_) => Ok(()),
              Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
              },
            };
          }
        }
      } // end extra cols mentioned in footer
    }
  }

  // footer
  if !datatype::is_na(&config.footer_option) {
    let _ = match stdout!("{: <6}", "") {
      Ok(_) => Ok(()),
      Err(e) => match e.kind() {
        std::io::ErrorKind::BrokenPipe => Ok(()),
        _ => Err(e),
      },
    };
    if config.is_tty || config.is_force_color {
      let _ = match stdoutln!(
        "{}",
        config.footer_option.truecolor(
          config.meta_color[0],
          config.meta_color[1],
          config.meta_color[2]
        )
      ) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    } else {
      let _ = match stdoutln!("{}", config.footer_option) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
          std::io::ErrorKind::BrokenPipe => Ok(()),
          _ => Err(e),
        },
      };
    }
  }
}

// how wide will the print be?
fn get_num_cols_to_print(cols: usize, vp: Vec<Vec<String>>, term_tuple: (u16, u16)) -> usize {
  let mut last = 0;
  let mut j = format!("{: <6}", "");
  for col in 0..cols {
    let text = vp[0].get(col).unwrap().to_string();
    j.push_str(&text);
    let total_width = j.chars().count();
    let term_width = term_tuple.0 as usize;
    if total_width > term_width {
      break;
    }
    last = col + 1;
  }
  last
}
