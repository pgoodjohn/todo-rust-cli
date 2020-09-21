use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};
use structopt::StructOpt;

// StructOps for parsing CLI arguments https://docs.rs/structopt/0.3.17/structopt/
#[derive(StructOpt)]
struct Cli {
    command: String,
    #[structopt(parse(from_os_str))]
    db: std::path::PathBuf,
    #[structopt(short = "t", long = "text")]
    text: Option<String>,
    #[structopt(short = "i", long = "id")]
    id: Option<i32>,
}

#[derive(Debug)]
struct Todo {
    id: i32,
    todo: String,
    checked: bool,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let conn = Connection::open(&args.db)?;

    if args.command == "help" {
        println!("Usage: help");
    } else if args.command == "init" {
        initDatabase(conn);
    } else if args.command == "list" {
        listTodos(conn);
    } else if args.command == "all" {
        listAllTodos(conn);
    } else if args.command == "check" {
        checkTodo(args.id.unwrap(), conn);
    } else if args.command == "add" {
        addTodo(args.text.unwrap(), conn);
    } else {
        panic!("Unknown command {}, exiting", args.command);
    };

    Ok(())
}

fn initDatabase(connection: Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id integer PRIMARY KEY,
            todo varchar(255) NOT NULL,
            checked BOOL DEFAULT false)",
        NO_PARAMS,
    )?;

    Ok(())
}

fn listAllTodos(connection: Connection) -> Result<()> {
    let mut stmt = connection.prepare("SELECT * FROM todos")?;

    let query = stmt.query_map(NO_PARAMS, |row| {
        Ok(Todo {
            id: row.get(0)?,
            todo: row.get(1)?,
            checked: row.get(2)?,
        })
    })?;

    for todo in query {
        // println!("{} {} {}", todo.id, todo.text, todo.checked);
        println!("{:?}", todo.unwrap())
    }

    Ok(())
}

fn checkTodo(todoIdentifier: i32, conn: Connection) -> Result<()> {
    conn.execute(
        "UPDATE todos SET checked = true WHERE id = ?1",
        params![todoIdentifier],
    )?;

    Ok(())
}

fn addTodo(text: String, conn: Connection) -> Result<()> {
    conn.execute("INSERT INTO todos (todo) VALUES(?1)", params![text])
        .unwrap();

    Ok(())
}

fn listTodos(conn: Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM todos WHERE checked = false")?;

    let query = stmt.query_map(NO_PARAMS, |row| {
        Ok(Todo {
            id: row.get(0)?,
            todo: row.get(1)?,
            checked: row.get(2)?,
        })
    })?;

    for todo in query {
        // println!("{} {} {}", todo.id, todo.text, todo.checked);
        println!("{:?}", todo.unwrap())
    }

    Ok(())
}
