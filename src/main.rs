/* use  **************************************************************************************************/

use anyhow::{self, Context, Result};
use clap;
use clap::Parser;
use md5::compute;
use nom::{
  bytes::complete::{tag, take},
  number::complete::be_u16,
  IResult,
};
use std::io::Write;
use std::str;
use std::time::SystemTime;

/* mod  **************************************************************************************************/

/* type alias  *******************************************************************************************/

/* global const  *****************************************************************************************/

const HEADER: &str = "path,compiled date,top,device,full file md5,body section md5,option,memo,";

/* trait  ************************************************************************************************/

/* enum  *************************************************************************************************/

/* struct  ***********************************************************************************************/

#[derive(Debug, Default, Parser)]
#[clap(version)]
struct Opt {
  #[clap(help = "If not given, the bit file in the current directory is used.")]
  bit_file: Option<String>,
  #[clap(short, long, help = "appends info to file as csv format")]
  append_to: Option<String>,
}

#[derive(Debug, Clone)]
struct Info {
  path: Option<String>,
  design: String,
  options: Vec<String>,
  device: String,
  datetime: String,
  md5_body_section: String,
  md5_full_file: String,
}

/* unsafe impl standard traits  **************************************************************************/

/* impl standard traits  *********************************************************************************/

impl std::fmt::Display for Info {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let path = self.path.clone().unwrap_or("".into());
    write!(
      f,
      "{},{},{},{},{},{},{},",
      path,
      self.datetime,
      self.design,
      self.device,
      self.md5_full_file,
      self.md5_body_section,
      self.options.join(";")
    )
  }
}

/* impl custom traits  ***********************************************************************************/

/* impl  *************************************************************************************************/

/* fn  ***************************************************************************************************/

fn newest_bit_file_in_current_directory() -> Option<String> {
  let mut bit_files = std::fs::read_dir(".")
    .unwrap()
    .filter_map(|entry| {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.is_file() {
        let path_str = path.file_name().unwrap().to_str().unwrap();
        if path_str.ends_with(".bit") {
          let timestamp = std::fs::metadata(path_str).unwrap().modified().unwrap();
          Some((timestamp, path_str.to_string()))
        } else {
          None
        }
      } else {
        None
      }
    })
    .collect::<Vec<(SystemTime, String)>>();
  bit_files.sort();
  bit_files.pop().map(|(_, bit_file)| bit_file)
}

fn md5_string(data: &[u8]) -> String {
  format!("{:x}", compute(data))
}

fn u16(input: &[u8]) -> IResult<&[u8], u16> {
  be_u16(input)
}

fn u8s(input: &[u8], len: usize) -> IResult<&[u8], &[u8]> {
  take(len)(input)
}

fn char_n(input: &[u8], len: usize) -> IResult<&[u8], String> {
  let (input, result) = take(len)(input)?;
  let result_str = str::from_utf8(result).unwrap().to_string();
  Ok((input, result_str))
}

fn char_x<'a>(input: &'a [u8], tag_val: &'static str) -> IResult<&'a [u8], &'a [u8]> {
  let x = tag(tag_val)(input)?;
  Ok(x)
}

fn trim_zero(input: String) -> String {
  let mut result = input.clone();
  result.pop();
  result
}

fn info(full_file: &[u8]) -> IResult<&[u8], Info> {
  let (design, device, date, body_only, time) = extract_info_seed(full_file)?;
  seed_to_info(design, body_only, device, date, time, full_file)
}

fn extract_info_seed(
  input: &[u8],
) -> Result<(String, String, String, &[u8], String), nom::Err<nom::error::Error<&[u8]>>> {
  let (input, len1) = u16(input)?;
  let (input, _header1) = u8s(input, len1 as usize)?;
  let (input, _len2) = u16(input)?;
  let (input, _) = char_x(input, "a")?;
  let (input, len3) = u16(input)?;
  let (input, design) = char_n(input, len3 as usize)?;
  let (input, _) = char_x(input, "b")?;
  let (input, len4) = u16(input)?;
  let (input, device) = char_n(input, len4 as usize)?;
  let (input, _) = char_x(input, "c")?;
  let (input, len5) = u16(input)?;
  let (input, date) = char_n(input, len5 as usize)?;
  let (input, _) = char_x(input, "d")?;
  let (input, len6) = u16(input)?;
  let (input, time) = char_n(input, len6 as usize)?;
  Ok((design, device, date, input, time))
}

fn seed_to_info<'a>(
  design: String,
  pure_body: &'a [u8],
  device: String,
  date: String,
  time: String,
  entire_file: &'a [u8],
) -> Result<(&'a [u8], Info), nom::Err<nom::error::Error<&'a [u8]>>> {
  let design = trim_zero(design);
  let top_design = design.split('.').next().unwrap().to_string();
  let options = design
    .split(';')
    .skip(1)
    .map(|x| x.into())
    .collect::<Vec<_>>();

  Ok((
    pure_body,
    Info {
      path: None,
      design: top_design.to_string(),
      options,
      device: trim_zero(device),
      datetime: format!("{} {}", trim_zero(date), trim_zero(time)),
      md5_body_section: md5_string(pure_body),
      md5_full_file: md5_string(entire_file),
    },
  ))
}

fn parse_path(path: &str) -> Result<Info> {
  let xs = std::fs::read(path)?;
  let (_, info) = info(&xs).map_err(|_e| anyhow::anyhow!("{path} parse error"))?;
  let file_name = std::path::Path::new(path)
    .file_name()
    .unwrap()
    .to_str()
    .unwrap();
  Ok(Info {
    path: Some(file_name.into()),
    ..info
  })
}

fn main() -> Result<()> {
  let Opt {
    bit_file,
    append_to,
  } = Opt::parse();

  let bit_file = bit_file
    .or_else(newest_bit_file_in_current_directory)
    .with_context(|| {
      "bit file not found. Please specify bit file path  or put bit file in current directory."
    })?;

  let info = parse_path(&bit_file).with_context(|| {
    format!(
      "Failed to parse bit file: {}",
      std::fs::canonicalize(&bit_file).unwrap().display()
    )
  })?;

  println!("{}", info);

  if let Some(append_to) = &append_to {
    let not_exists = !std::path::Path::new(append_to).exists();
    let mut f = std::fs::OpenOptions::new()
      .create(true)
      .append(true)
      .open(append_to)
      .with_context(|| {
        format!(
          "Failed to open file: {}",
          std::fs::canonicalize(&append_to).unwrap().display()
        )
      })?;
    if not_exists {
      writeln!(f, "{}", HEADER)?;
    }
    writeln!(f, "{}", info)?;
  }

  Ok(())
}

/* async fn  *********************************************************************************************/

/* test for pri ******************************************************************************************/

/* test for ******************************************************************************************/
