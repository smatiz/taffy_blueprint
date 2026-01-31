use taffy::prelude::*;

pub fn compare_style(s1: &Style, s2: &Style) -> bool {
    if s1.display != s2.display {
        println!("display: {:?} \n!=\n {:?}", s1.display, s2.display);
        return false;
    }
    if s1.item_is_table != s2.item_is_table {
        println!(
            "item_is_table: {:?} \n!=\n {:?}",
            s1.item_is_table, s2.item_is_table
        );
        return false;
    }
    if s1.item_is_replaced != s2.item_is_replaced {
        println!(
            "item_is_replaced: {:?} \n!=\n {:?}",
            s1.item_is_replaced, s2.item_is_replaced
        );
        return false;
    }
    if s1.box_sizing != s2.box_sizing {
        println!("box_sizing: {:?} \n!=\n {:?}", s1.box_sizing, s2.box_sizing);
        return false;
    }
    if s1.overflow != s2.overflow {
        println!("overflow: {:?} \n!=\n {:?}", s1.overflow, s2.overflow);
        return false;
    }
    if s1.scrollbar_width != s2.scrollbar_width {
        println!(
            "scrollbar_width: {:?} \n!=\n {:?}",
            s1.scrollbar_width, s2.scrollbar_width
        );
        return false;
    }
    if s1.position != s2.position {
        println!("position: {:?} \n!=\n {:?}", s1.position, s2.position);
        return false;
    }
    if s1.inset != s2.inset {
        println!("inset: {:?} \n!=\n {:?}", s1.inset, s2.inset);
        return false;
    }
    if s1.size != s2.size {
        println!("size: {:?} \n!=\n {:?}", s1.size, s2.size);
        return false;
    }
    if s1.min_size != s2.min_size {
        println!("min_size: {:?} \n!=\n {:?}", s1.min_size, s2.min_size);
        return false;
    }
    if s1.max_size != s2.max_size {
        println!("max_size: {:?} \n!=\n {:?}", s1.max_size, s2.max_size);
        return false;
    }
    if s1.aspect_ratio != s2.aspect_ratio {
        println!(
            "aspect_ratio: {:?} \n!=\n {:?}",
            s1.aspect_ratio, s2.aspect_ratio
        );
        return false;
    }
    if s1.margin != s2.margin {
        println!("margin: {:?} \n!=\n {:?}", s1.margin, s2.margin);
        return false;
    }
    if s1.padding != s2.padding {
        println!("padding: {:?} \n!=\n {:?}", s1.padding, s2.padding);
        return false;
    }
    if s1.border != s2.border {
        println!("border: {:?} \n!=\n {:?}", s1.border, s2.border);
        return false;
    }
    if s1.align_items != s2.align_items {
        println!(
            "align_items: {:?} \n!=\n {:?}",
            s1.align_items, s2.align_items
        );
        return false;
    }
    if s1.align_self != s2.align_self {
        println!("align_self: {:?} \n!=\n {:?}", s1.align_self, s2.align_self);
        return false;
    }
    if s1.justify_items != s2.justify_items {
        println!(
            "justify_items: {:?} \n!=\n {:?}",
            s1.justify_items, s2.justify_items
        );
        return false;
    }
    if s1.justify_self != s2.justify_self {
        println!(
            "justify_self: {:?} \n!=\n {:?}",
            s1.justify_self, s2.justify_self
        );
        return false;
    }
    if s1.align_content != s2.align_content {
        println!(
            "align_content: {:?} \n!=\n {:?}",
            s1.align_content, s2.align_content
        );
        return false;
    }
    if s1.justify_content != s2.justify_content {
        println!(
            "justify_content: {:?} \n!=\n {:?}",
            s1.justify_content, s2.justify_content
        );
        return false;
    }
    if s1.gap != s2.gap {
        println!("gap: {:?} \n!=\n {:?}", s1.gap, s2.gap);
        return false;
    }
    if s1.text_align != s2.text_align {
        println!("text_align: {:?} \n!=\n {:?}", s1.text_align, s2.text_align);
        return false;
    }
    if s1.flex_direction != s2.flex_direction {
        println!(
            "flex_direction: {:?} \n!=\n {:?}",
            s1.flex_direction, s2.flex_direction
        );
        return false;
    }
    if s1.flex_wrap != s2.flex_wrap {
        println!("flex_wrap: {:?} \n!=\n {:?}", s1.flex_wrap, s2.flex_wrap);
        return false;
    }
    if s1.flex_basis != s2.flex_basis {
        println!("flex_basis: {:?} \n!=\n {:?}", s1.flex_basis, s2.flex_basis);
        return false;
    }
    if s1.flex_grow != s2.flex_grow {
        println!("flex_grow: {:?} \n!=\n {:?}", s1.flex_grow, s2.flex_grow);
        return false;
    }
    if s1.flex_shrink != s2.flex_shrink {
        println!(
            "flex_shrink: {:?} \n!=\n {:?}",
            s1.flex_shrink, s2.flex_shrink
        );
        return false;
    }
    if s1.grid_template_rows != s2.grid_template_rows {
        println!(
            "grid_template_rows: {:?} \n!=\n {:?}",
            s1.grid_template_rows, s2.grid_template_rows
        );
        return false;
    }
    if s1.grid_template_columns != s2.grid_template_columns {
        println!(
            "grid_template_columns: {:?} \n!=\n {:?}",
            s1.grid_template_columns, s2.grid_template_columns
        );
        return false;
    }
    if s1.grid_auto_rows != s2.grid_auto_rows {
        println!(
            "grid_auto_rows: {:?} \n!=\n {:?}",
            s1.grid_auto_rows, s2.grid_auto_rows
        );
        return false;
    }
    if s1.grid_auto_columns != s2.grid_auto_columns {
        println!(
            "grid_auto_columns: {:?} \n!=\n {:?}",
            s1.grid_auto_columns, s2.grid_auto_columns
        );
        return false;
    }
    if s1.grid_auto_flow != s2.grid_auto_flow {
        println!(
            "grid_auto_flow: {:?} \n!=\n {:?}",
            s1.grid_auto_flow, s2.grid_auto_flow
        );
        return false;
    }
    if s1.grid_template_areas != s2.grid_template_areas {
        println!(
            "grid_template_areas: {:?} \n!=\n {:?}",
            s1.grid_template_areas, s2.grid_template_areas
        );
        return false;
    }
    if s1.grid_template_column_names != s2.grid_template_column_names {
        println!(
            "grid_template_column_names: {:?} \n!=\n {:?}",
            s1.grid_template_column_names, s2.grid_template_column_names
        );
        return false;
    }
    if s1.grid_template_row_names != s2.grid_template_row_names {
        println!(
            "grid_template_row_names: {:?} \n!=\n {:?}",
            s1.grid_template_row_names, s2.grid_template_row_names
        );
        return false;
    }
    if s1.grid_row != s2.grid_row {
        println!("grid_row: {:?} \n!=\n {:?}", s1.grid_row, s2.grid_row);
        return false;
    }
    if s1.grid_column != s2.grid_column {
        println!(
            "grid_column: {:?} \n!=\n {:?}",
            s1.grid_column, s2.grid_column
        );
        return false;
    }
    true
}
