initSidebarItems({"enum":[["Date","A wrapper that can be used to represent infinity with `Type::Date` types."],["IsNull","An enum representing the nullability of a Postgres value."],["Kind","Represents the kind of a Postgres type."],["Timestamp","A wrapper that can be used to represent infinity with `Type::Timestamp` and `Type::Timestamptz` types."]],"macro":[["accepts","Generates a simple implementation of `ToSql::accepts` which accepts the types passed to it."],["to_sql_checked","Generates an implementation of `ToSql::to_sql_checked`."]],"struct":[["Field","Information about a field of a composite type."],["Json","A wrapper type to allow arbitrary `Serialize`/`Deserialize` types to convert to Postgres JSON values."],["Type","A Postgres type."],["WasNull","An error indicating that a `NULL` Postgres value was passed to a `FromSql` implementation that does not support `NULL` values."],["WrongType","An error indicating that a conversion was attempted between incompatible Rust and Postgres types."]],"trait":[["FromSql","A trait for types that can be created from a Postgres value."],["FromSqlOwned","A trait for types which can be created from a Postgres value without borrowing any data."],["ToSql","A trait for types that can be converted into Postgres values."]],"type":[["Oid","A Postgres OID."]]});