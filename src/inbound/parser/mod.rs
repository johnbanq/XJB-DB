pub mod test;

use pest::iterators::Pair;
use pest::Parser;

use crate::inbound::ast::*;
use pest::error::Error;
use std::str::FromStr;

pub fn parse_sql(input: &str) -> Result<Query, Error<Rule>> {
    let parsed = SQLParser::parse(Rule::query, input)?.next().unwrap();
    Ok(parse_value(parsed))
}

#[derive(Parser)]
#[grammar = "inbound/parser/sql.pest"]
struct SQLParser;

fn parse_value(pair: Pair<Rule>) -> Query {
    match pair.as_rule() {
        Rule::create_table => {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap();
            let fields = pairs.next().unwrap();
            Query::CreateTable(CreateTableQuery{
                name: String::from(name.as_str()),
                fields: parse_select_fields(fields)
            })
        }
        Rule::insert => {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap();
            let values = pairs.next().unwrap();
            Query::Insert(InsertQuery{
                table_name: String::from(name.as_str()),
                values: parse_insert_values(values)
            })
        }
        Rule::select => {
            let mut pairs = pair.into_inner();
            let fields = pairs.next().unwrap();
            let from_clause = pairs.next().unwrap();
            Query::Select(SelectQuery{
                table_name:  parse_select_from_clause(from_clause),
                fields: parse_select_field_clause(fields)
            })
        }
        Rule::delete => {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap();
            Query::Delete(DeleteQuery{
                table_name: String::from(name.as_str())
            })
        }
        Rule::drop_table => {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap();
            Query::DropTable(DropTableQuery{
                table_name: String::from(name.as_str())
            })
        }
        _ => panic!("not implemented")
    }
}

fn parse_select_fields(pair: Pair<Rule>) -> Vec<CreateTableField> {
    pair.into_inner()
        .map(|p| {
            let mut field_pairs = p.into_inner();
            let name = field_pairs.next().unwrap();
            let data_type = field_pairs.next().unwrap();
            CreateTableField{ name: String::from(name.as_str()), data_type: String::from(data_type.as_str()) }
        })
        .collect()
}

fn parse_insert_values(pair: Pair<Rule>) -> Vec<DataLiteral> {
    pair.into_inner()
        .map(|p| {
            let mut field_pairs = p.into_inner();
            let value = field_pairs.next().unwrap();
            match value.as_rule() {
                //note: we need to get rid of the " prefix & suffxi, hence the slicing
                Rule::string => DataLiteral::String(String::from(&(value.as_str())[1..value.as_str().len()-1])),
                Rule::number => DataLiteral::Number(i64::from_str(value.as_str()).expect("invalid integer literal!")),
                _ => panic!("unknown literal rule type!")
            }
        })
        .collect()
}

fn parse_select_field_clause(pair: Pair<Rule>) -> SelectField {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::select_field_clause_all => SelectField::All(),
        Rule::select_field_clause_field_list => SelectField::Fields(
            pair.into_inner()
            .map(|p| String::from(p.as_str()) )
            .collect()
        ),
        _ => panic!("unknown select field clause!")
    }
}

fn parse_select_from_clause(pair: Pair<Rule>) -> String {
    let pairs = pair.into_inner().next().unwrap();
    String::from(pairs.as_str())
}