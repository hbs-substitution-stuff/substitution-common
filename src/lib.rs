pub mod util;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Read;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub trait SubstitutionPDFExtractor {
	fn schedule_from_pdf<R: Read>(pdf: R) -> Result<SubstitutionSchedule, Box<dyn std::error::Error>>;
}

/// One column with Substitutions from the PDF
#[derive(Serialize, Deserialize, PartialOrd, PartialEq, Debug)]
pub struct SubstitutionColumn {
	#[serde(rename(serialize = "0"))]
	#[serde(rename(deserialize = "0"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub block_0: Option<String>,
	#[serde(rename(serialize = "1"))]
	#[serde(rename(deserialize = "1"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub block_1: Option<String>,
	#[serde(rename(serialize = "2"))]
	#[serde(rename(deserialize = "2"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub block_2: Option<String>,
	#[serde(rename(serialize = "3"))]
	#[serde(rename(deserialize = "3"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub block_3: Option<String>,
	#[serde(rename(serialize = "4"))]
	#[serde(rename(deserialize = "4"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub block_4: Option<String>,
	#[serde(rename(serialize = "5"))]
	#[serde(rename(deserialize = "5"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub block_5: Option<String>,
}

/// Represents a column from the substitution PDF.
/// Does not include the class name, only the substitutions.
impl SubstitutionColumn {
	pub fn new() -> Self {
		Self {
			block_0: None,
			block_1: None,
			block_2: None,
			block_3: None,
			block_4: None,
			block_5: None,
		}
	}

	pub fn from_vec(col: Vec<String>) -> Self {
		let mut column = col.into_iter().map(|s| {
			if !s.chars().all(|c| c == ' ' || c == '\n') {
				Some(s)
			} else {
				None
			}
		});

		Self {
			block_0: column.next().unwrap(),
			block_1: column.next().unwrap(),
			block_2: column.next().unwrap(),
			block_3: column.next().unwrap(),
			block_4: column.next().unwrap(),
			block_5: column.next().unwrap(),

		}
	}
}

impl Default for SubstitutionColumn {
	fn default() -> Self {
		Self::new()
	}
}

impl Display for SubstitutionColumn {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
	}
}

/// Contains the extracted PDF data of the schedule PDF
#[derive(Serialize, Deserialize, Debug)]
pub struct SubstitutionSchedule {
	/// The creation date inside the PDF in milliseconds.
	pub pdf_issue_date: i64,
	/// The name of the class is the Key and the Value is a Substitutions struct.
	pub entries: HashMap<String, SubstitutionColumn>,
}

#[derive(Error, Debug)]
pub enum PDFJsonError {
	#[error("There was an error while reading the PDF File.")]
	PDFReadError
}

