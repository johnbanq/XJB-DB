#[cfg(test)]
pub mod test {

    use crate::inbound::parser::parse_sql;
    use crate::inbound::ast::*;
    use crate::inbound::ast::SelectField::{All, Fields};

    #[test]
    fn parse_sql_works_for_create_table() {
        let parsed = parse_sql(
            " \
            CREATE TABLE test ( \
                string VARCHAR,
                int BIGINT
            );\
            "
        ).unwrap();

        if let Query::CreateTable(query) = parsed {
            assert_eq!(
                query,
                CreateTableQuery{
                    name: String::from("test"),
                    fields: vec![
                        CreateTableField { name: String::from("string"), data_type: String::from("VARCHAR") },
                        CreateTableField { name: String::from("int"), data_type: String::from("BIGINT") }
                    ]
                }
            );
        } else {
            panic!("wrong query type: {:?}", parsed);
        }
    }

    #[test]
    fn parse_sql_works_for_insert() {
        let parsed = parse_sql(
            " \
            INSERT INTO test VALUES (\"test\", 123);\
            "
        ).unwrap();

        if let Query::Insert(query) = parsed {
            assert_eq!(
                query,
                InsertQuery {
                    table_name: String::from("test"),
                    values: vec![
                        DataLiteral::String(String::from("test")),
                        DataLiteral::Number(123)
                    ]
                }
            );
        } else {
            panic!("wrong query type: {:?}", parsed);
        }
    }

    #[test]
    fn parse_sql_works_for_select_all() {
        let parsed = parse_sql(
            " \
            SELECT * FROM test;\
            "
        ).unwrap();

        if let Query::Select(query) = parsed {
            assert_eq!(
                query,
                SelectQuery {
                    table_name: String::from("test"),
                    fields: All()
                }
            );
        } else {
            panic!("wrong query type: {:?}", parsed);
        }
    }

    #[test]
    fn parse_sql_works_for_select_fields() {
        let parsed = parse_sql(
            " \
            SELECT a, b FROM test;\
            "
        ).unwrap();

        if let Query::Select(query) = parsed {
            assert_eq!(
                query,
                SelectQuery {
                    table_name: String::from("test"),
                    fields: Fields(vec![String::from("a"), String::from("b")])
                }
            );
        } else {
            panic!("wrong query type: {:?}", parsed);
        }
    }

    #[test]
    fn parse_sql_works_for_delete() {
        let parsed = parse_sql(
            " \
            DELETE FROM test;\
            "
        ).unwrap();

        if let Query::Delete(query) = parsed {
            assert_eq!(
                query,
                DeleteQuery {
                    table_name: String::from("test")
                }
            );
        } else {
            panic!("wrong query type: {:?}", parsed);
        }
    }

    #[test]
    fn parse_sql_works_for_drop_table() {
        let parsed = parse_sql(
            " \
            DROP TABLE test;\
            "
        ).unwrap();

        if let Query::DropTable(query) = parsed {
            assert_eq!(
                query,
                DropTableQuery {
                    table_name: String::from("test")
                }
            );
        } else {
            panic!("wrong query type: {:?}", parsed);
        }
    }

}