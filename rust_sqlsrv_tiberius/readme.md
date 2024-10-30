## Minimal test for connecting to SQL Server from Rust using Tiberius

A Rust based experiment to test the behavior that it was initially faced and described in [this mssql-jdbc](https://github.com/microsoft/mssql-jdbc/issues/2510) issue.

<br/>

### Usage

1. Set the `JDBC_CONN_STRING` in your `.env` file.<br/>
   Example: `JDBC_CONN_STRING="jdbc:sqlserver://my-sql-server:1433;database=test;user=myUser;password=myPass;trustServerCertificate=true;sendStringParametersAsUnicode=false"`
1. Use `cargo run`
