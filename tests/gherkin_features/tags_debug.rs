use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/parser/gherkin.pest"]
struct TestParser;

fn main() {
    let input = "@smoke @api\nFeature: Test\n";
    match TestParser::parse(Rule::feature, input) {
        Ok(pairs) => {
            println!("Parse successful!");
            for pair in pairs {
                println!("Rule: {:?}", pair.as_rule());
                for inner in pair.into_inner() {
                    println!("  Inner: {:?} = '{}'", inner.as_rule(), inner.as_str());
                }
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}
