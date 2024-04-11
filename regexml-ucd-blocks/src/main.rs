fn main() {
    let blocks_txt = include_str!("Blocks.txt");

    println!(
        r#"
pub(crate) struct Block {{
    pub(crate) name: &'static str,
    pub(crate) start: u32,
    pub(crate) end: u32,
}}

"#
    );
    // go through lines in blocks, skipping comments that start with '#'
    let mut all_blocks = vec![];
    for line in blocks_txt.lines() {
        if line.starts_with('#') {
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }

        // split line into fields
        let fields: Vec<&str> = line.split(';').collect();

        // get the range split by .. from the first field
        let range: Vec<&str> = fields[0].split("..").collect();
        // turn range into start and end as u32
        let start = u32::from_str_radix(range[0], 16).unwrap();
        let end = u32::from_str_radix(range[1], 16).unwrap();

        // get the second field, strip whitespace
        let block_name = fields[1].trim();
        // const name is block name in uppercase with _ instead of space and -
        let const_name = block_name.to_uppercase().replace([' ', '-'], "_");

        println!(
            r#"
pub(crate) const {}: Block = Block {{
    name: "{}", 
    start: 0x{:X}, 
    end: 0x{:X}
}};
"#,
            const_name, block_name, start, end
        );
        all_blocks.push(const_name);
        // print the code point and block name
        // println!("{}..{}; {}", start, end, block_name);
    }
    println!(
        "pub(crate) const ALL_BLOCKS: &[Block] = &[{}];",
        all_blocks.join(", ")
    );
}
