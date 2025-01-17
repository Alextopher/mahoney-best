use std::collections::HashMap;

use crate::{Color, Rectangle};

pub fn create_program(
    colors: &[Color],
    instructions: &HashMap<Color, Vec<Rectangle>>,
    scale: usize,
    width: usize,
    height: usize,
) -> String {
    let mut program = String::new();
    program.push_str("/// THIS FILE WAS COMPUTER GENERATED\n");
    program.push_str("function art() {\n");
    for color in colors.iter() {
        program.push_str(&format!(
            "\tfill({}, {}, {});\n",
            color[0], color[1], color[2]
        ));
        for instruction in instructions.get(color).unwrap() {
            program.push_str(&format!(
                "\trect({}, {}, {}, {});\n",
                instruction.x * scale,
                instruction.y * scale,
                instruction.width * scale,
                instruction.height * scale
            ));
        }
        program.push('\n');
    }
    program.pop();
    program.push_str("}\n\n");
    program.push_str("function setup() {\n");
    program.push_str(&format!(
        "\tcreateCanvas({}, {});\n",
        width * scale,
        height * scale
    ));
    program.push_str("\tbackground(0);\n");
    program.push_str("\tnoStroke();\n");
    program.push_str("\tart();\n");
    program.push_str("}\n");
    program.push('\n');
    program.push_str("var strokeState = false;\n");
    program.push_str("function keyPressed() {\n");
    program.push_str("\tif (keyCode == BACKSPACE) {\n");
    program.push_str("\t\tstrokeState = !strokeState;\n");
    program.push_str("\t\tif (strokeState) {\n");
    program.push_str("\t\t\tstroke(0, 0, 0);\n");
    program.push_str("\t\t} else {\n");
    program.push_str("\t\t\tnoStroke();\n");
    program.push_str("\t\t}\n");
    program.push_str("\t\tart();\n");
    program.push_str("\t}\n");
    program.push_str("}\n");
    program
}
