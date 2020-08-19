
#[derive(Debug)]
#[derive(PartialEq)]
pub enum DataLiteral {
    String(String),
    Number(i64)
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Query{
    CreateTable(CreateTableQuery),
    Insert(InsertQuery),
    Select(SelectQuery),
    Delete(DeleteQuery),
    DropTable(DropTableQuery)
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct CreateTableQuery {
    pub name: String,
    pub fields: Vec<CreateTableField>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct CreateTableField {
    pub name: String,
    pub data_type: String
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct InsertQuery {
    pub table_name: String,
    pub values: Vec<DataLiteral>
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct SelectQuery {
    pub table_name: String,
    pub fields: SelectField
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SelectField {
    All(),
    Fields(Vec<String>)
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct DeleteQuery {
    pub table_name: String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct DropTableQuery {
    pub table_name: String
}