use prettytable::{format::consts::FORMAT_CLEAN, Table};
use std::fmt::Display;

/// Prevents a [DisplayTable] to implement [Display] as long as the with_format
/// method is not called.
pub struct NoFormatFn;

/// Displays the inner data into a table.
pub struct DisplayTable<'a, T, FormatFn> {
    item: &'a T,
    format_fn: FormatFn,
}

impl<'a, T, FormatFn> Display for DisplayTable<'a, T, FormatFn>
where
    FormatFn: Fn(&mut Table, &T),
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);

        let fmt = &self.format_fn;
        fmt(&mut table, self.item);

        write!(f, "{}", table)
    }
}

impl<'a, T> DisplayTable<'a, T, NoFormatFn> {
    pub fn new(item: &'a T) -> Self {
        Self {
            item,
            format_fn: NoFormatFn,
        }
    }
}

impl<'a, T, FormatFn> DisplayTable<'a, T, FormatFn> {
    /// Sets the table's content format function.
    pub fn with_format<F>(self, format_fn: F) -> DisplayTable<'a, T, F>
    where
        F: Fn(&mut Table, &T),
    {
        DisplayTable {
            item: self.item,
            format_fn,
        }
    }
}
