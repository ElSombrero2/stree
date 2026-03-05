use comfy_table::{Table, presets};

pub fn get_table() -> Table {
  let mut table = Table::new();
  table.load_preset(presets::NOTHING);
  table
}