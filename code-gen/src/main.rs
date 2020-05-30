use indoc::indoc;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut css_file = File::create("../ui/src/view/style/gen.css")?;
    css_file.write_all(css().as_bytes())?;

    let mut style_constants_file = File::create("../ui/src/view/style/gen_const.rs")?;
    style_constants_file.write_all(style_constants().as_bytes())?;

    Ok(())
}

static MAX_STYLE_SIZE: u8 = 8;
static SIDES: [&str; 4] = ["top", "bottom", "left", "right"];

fn indent(buf: &mut String, size: u8, with_text: &str) {
    let tab: String = (0..(4 * size)).map(|_| " ").collect();

    buf.push_str(tab.as_str());
    buf.push_str(with_text);
}





////////////////////////////////////////////////////////////////
// STYLE CODE //
////////////////////////////////////////////////////////////////

fn style_constants() -> String {
    let mut buf: String = String::new();

    buf.push_str(indoc!(
        r#"
        ///
        /// Do not edit this file by hand! This file was generated. Change the script that
        /// generates this code instead and re-run it.
        ///
        /// The script is located in code-gen/src/main.rs
        ///
        
        "#
    ));

    let mut size = 0;

    while size < MAX_STYLE_SIZE {
        let mut side_index = 0;

        while side_index < SIDES.len() {
            let side = SIDES[side_index];

            buf.push_str(spacing_constant("margin", side, size).as_str());
            buf.push('\n');
            buf.push_str(spacing_constant("padding", side, size).as_str());
            buf.push('\n');

            side_index += 1;
        }

        size += 1;
    }

    for side in SIDES.iter() {
        buf.push_str(spacing_function("margin", side).as_str());
        buf.push('\n');
        buf.push_str(spacing_function("padding", side).as_str());
        buf.push('\n');
    }

    buf
}

fn spacing_function(has_side: &str, side: &str) -> String {
    let mut buf: String = String::new();

    buf.push_str(
        format!(
            "pub fn {}(size:u8) -> &'static str {{\n",
            format!("{}_{}", has_side, side),
        )
        .as_str(),
    );

    indent(&mut buf, 1, "match size {\n");

    let mut size: u8 = 0;

    while size < MAX_STYLE_SIZE {
        indent(
            &mut buf,
            2,
            format!(
                "{} => {}_{}_{},\n",
                size,
                has_side.to_ascii_uppercase(),
                side.to_ascii_uppercase(),
                size
            )
            .as_str(),
        );

        size += 1;
    }

    indent(
        &mut buf,
        2,
        format!(
            "_ => {}_{}_0",
            has_side.to_ascii_uppercase(),
            side.to_ascii_uppercase()
        )
        .as_str(),
    );

    buf.push('\n');
    indent(&mut buf, 1, "}\n");
    buf.push('}');

    buf
}

fn spacing_constant(has_side: &str, side: &str, size: u8) -> String {
    format!(
        "pub static {}: &'static str = {};",
        format!(
            "{}_{}_{}",
            has_side.to_ascii_uppercase(),
            side.to_ascii_uppercase(),
            size
        ),
        format!("\"{}-{}-{}\"", has_side, side, size)
    )
}

////////////////////////////////////////////////////////////////
// CSS //
////////////////////////////////////////////////////////////////

fn css() -> String {
    let mut buf: String = String::new();

    buf.push_str(indoc!(
        r#"
        /*
        Do not edit this file by hand! This file was generated. Change the script that
        generates this css instead and re-run it.
        
        The script is located in code-gen/src/main.rs
        */
        
        "#
    ));

    let mut size = 0;

    while size < MAX_STYLE_SIZE {
        let mut side_index = 0;

        while side_index < SIDES.len() {
            let side = SIDES[side_index];

            buf.push_str(side_style_str("margin", side, size).as_str());
            buf.push_str("\n\n");
            buf.push_str(side_style_str("padding", side, size).as_str());
            buf.push_str("\n\n");

            side_index += 1;
        }

        size += 1;
    }

    buf
}

fn side_style_str(has_side: &str, side: &str, size: u8) -> String {
    let base_number: u32 = 2;
    format!(
        "{}{}",
        format!(".{}-{}-{} {{\n", has_side, side, size),
        format!(
            "    {}-{}: {}px;\n}}",
            has_side,
            side,
            base_number.pow(size as u32)
        )
    )
}
