use axum::{Router, extract::Multipart, http::StatusCode, response::IntoResponse, routing::post};
use calamine::{Data, Reader, Xlsx};
use serde::Serialize;
use std::io::Cursor;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Workbook {
    sheets: Vec<SheetMetadata>,
    cells: Vec<CellData>,
    merged_ranges: Vec<MergedRange>,
}

#[derive(Serialize)]
struct SheetMetadata {
    name: String,
    index: usize,
    hidden: bool,
}

#[derive(Serialize)]
struct CellData {
    sheet: String,
    address: String,
    row: u32,
    col: u32,
    data_type: String,
    value: String,
    formula: Option<String>,
}

#[derive(Serialize)]
struct MergedRange {
    sheet: String,
    start: String,
    end: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/convert", post(convert_handler));

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind address");
    let addr = listener.local_addr().unwrap();
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn convert_handler(mut multipart: Multipart) -> impl IntoResponse {
    let mut format_opt: Option<String> = None;
    let mut file_bytes = Vec::new();
    let mut filename_opt: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("format") => {
                let data = field.bytes().await.unwrap();
                format_opt = Some(String::from_utf8(data.to_vec()).unwrap());
            }
            Some("file") => {
                filename_opt = field.file_name().map(ToString::to_string);
                let data = field.bytes().await.unwrap();
                file_bytes = data.to_vec();
            }
            _ => {}
        }
    }

    let format = match format_opt {
        Some(f) => f,
        None => return (StatusCode::BAD_REQUEST, "Missing 'format'").into_response(),
    };
    let filename = match filename_opt {
        Some(f) => f,
        None => return (StatusCode::BAD_REQUEST, "Missing file name").into_response(),
    };

    let workbook = match parse_workbook(&file_bytes, &filename) {
        Ok(wb) => wb,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };

    let body = match format.as_str() {
        "json" => serde_json::to_string_pretty(&workbook).unwrap(),
        "yaml" => serde_yaml::to_string(&workbook).unwrap(),
        "xml" => serde_xml_rs::to_string(&workbook).unwrap(),
        "sql" => to_sql(&workbook),
        _ => return (StatusCode::BAD_REQUEST, "Unsupported format").into_response(),
    };

    let content_type = match format.as_str() {
        "json" => "application/json",
        "yaml" => "application/x-yaml",
        "xml" => "application/xml",
        "sql" => "text/plain",
        _ => "text/plain",
    };

    ([("Content-Type", content_type)], body).into_response()
}

fn parse_workbook(bytes: &[u8], filename: &str) -> Result<Workbook, String> {
    if filename.to_lowercase().ends_with(".csv") {
        parse_csv(bytes)
    } else {
        parse_excel(bytes)
    }
}

fn parse_csv(bytes: &[u8]) -> Result<Workbook, String> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(Cursor::new(bytes));
    let mut cells = Vec::new();

    for (row_idx, record) in rdr.records().enumerate() {
        let record = record.map_err(|e| e.to_string())?;
        for (col_idx, field) in record.iter().enumerate() {
            let address = format!(
                "{}{}",
                col_to_letter((col_idx + 1) as u32),
                row_idx as u32 + 1
            );
            cells.push(CellData {
                sheet: "Sheet1".into(),
                address: address.clone(),
                row: row_idx as u32 + 1,
                col: col_idx as u32 + 1,
                data_type: "String".into(),
                value: field.to_string(),
                formula: None,
            });
        }
    }

    Ok(Workbook {
        sheets: vec![SheetMetadata {
            name: "Sheet1".into(),
            index: 0,
            hidden: false,
        }],
        cells,
        merged_ranges: Vec::new(),
    })
}

fn parse_excel(bytes: &[u8]) -> Result<Workbook, String> {
    let mut excel =
        Xlsx::new(Cursor::new(bytes)).map_err(|e| format!("Excel open error: {}", e))?;

    let mut sheets = Vec::new();
    let mut cells = Vec::new();
    let merged_ranges = Vec::new();

    for (idx, name) in excel.sheet_names().iter().enumerate() {
        sheets.push(SheetMetadata {
            name: name.clone(),
            index: idx,
            hidden: false,
        });

        let range = excel
            .worksheet_range(name)
            .map_err(|e| format!("Error reading sheet {}: {}", name, e))?;

        for (r, c, v) in range.cells() {
            let address = format!("{}{}", col_to_letter(c as u32 + 1), r as u32 + 1);
            let (data_type, value, formula) = match *v {
                Data::Empty => continue,
                Data::String(ref s) => ("String".to_string(), s.clone(), None),
                Data::Float(f) => ("Number".to_string(), f.to_string(), None),
                Data::Int(i) => ("Number".to_string(), i.to_string(), None),
                Data::Bool(b) => ("Boolean".to_string(), b.to_string(), None),
                Data::Error(ref e) => ("Error".to_string(), format!("{:?}", e), None),
                Data::DateTime(dt) => ("DateTime".to_string(), dt.to_string(), None),
                Data::DateTimeIso(ref s) => ("DateTimeIso".to_string(), s.clone(), None),
                Data::DurationIso(ref s) => ("DurationIso".to_string(), s.clone(), None),
            };
            cells.push(CellData {
                sheet: name.clone(),
                address,
                row: r as u32 + 1,
                col: c as u32 + 1,
                data_type,
                value,
                formula,
            });
        }
    }

    Ok(Workbook {
        sheets,
        cells,
        merged_ranges,
    })
}

fn col_to_letter(mut col: u32) -> String {
    let mut s = String::new();
    while col > 0 {
        let rem = (col - 1) % 26;
        s.insert(0, (b'A' + rem as u8) as char);
        col = (col - 1) / 26;
    }
    s
}

fn to_sql(wb: &Workbook) -> String {
    let mut sql = String::new();
    sql.push_str(
        "CREATE TABLE cell_data (sheet TEXT, address TEXT, row INTEGER, col INTEGER, data_type TEXT, value TEXT, formula TEXT);\n",
    );
    for cell in &wb.cells {
        let formula_value = match &cell.formula {
            Some(f) => format!("'{}'", f.replace('\'', "''")),
            None => "NULL".into(),
        };
        sql.push_str(&format!(
            "INSERT INTO cell_data VALUES ('{}','{}',{},{},'{}','{}',{});\n",
            cell.sheet.replace('\'', "''"),
            cell.address,
            cell.row,
            cell.col,
            cell.data_type.replace('\'', "''"),
            cell.value.replace('\'', "''"),
            formula_value
        ));
    }
    sql
}
