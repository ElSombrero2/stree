use comfy_table::Table;

pub fn get_table(presets: &str) -> Table {
  let mut table = Table::new();
  table.load_preset(presets);
  table
}