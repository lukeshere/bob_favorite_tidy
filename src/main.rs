use rusqlite::{Connection, Result};
use std::{env::home_dir, fs};

fn main() -> Result<()> {
    const SQL:&str = "
    delete
    from translate_translate_favorite
    where translate_translate_favorite.localId in (
        select a.localId
        from translate_translate_favorite a
                 join (
            select localId, queryText from translate_translate_favorite group by queryText having count(queryText) > 1
        ) b on a.localId <> b.localId and a.queryText = b.queryText
    )
    ";
    const PATH: &str = "/Library/Application Support/com.ripperhe.Bob/VirtualSandbox/Documents";
    const FILE_NAME: &str = "bob-core.sqlite";

    let home_path = home_dir().unwrap();
    let home_path = home_path.to_str().unwrap();

    for item in fs::read_dir(format!("{}{}", home_path, PATH)).unwrap() {
        let item = item.unwrap();
        if item.file_name().to_str().unwrap() == FILE_NAME {
            let conn = Connection::open(item.path().to_str().unwrap()).unwrap();
            conn.execute(SQL, [])?;
        }
    }
    Ok(())
}
