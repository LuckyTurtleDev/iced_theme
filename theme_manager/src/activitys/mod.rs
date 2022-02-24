pub mod select;
use super::Window;

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(PartialEq)]
pub enum Activity {
	Select,
}
