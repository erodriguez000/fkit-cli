#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("This directory does not seem to be a awesome-app directory. Make sure to run awesome-app from the root project directory.")]
	DirNotValid,

	#[error("Fail to execute {0} cause: {1}")]
	Exec(String, String),

	#[error("Path not safe to delete {0}")]
	PathNotSafeToDelete(String),

	#[error("Directory {0} already exist. Cancelling.")]
	DirAlreadyExist(String),

	#[error("git command line not found. Required for awesome-app.")]
	GitNotPresent,

	#[error("Fail to read line")]
	StdinFailToReadLine,

	#[error(transparent)]
	IOError(#[from] std::io::Error),
}
