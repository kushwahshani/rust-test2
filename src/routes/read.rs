use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use worker::{Request, Response, Result, RouteContext};

#[derive(Deserialize, Serialize)]
pub struct StudentData {
    id: i32,
    name: String,
    age: i32,
}

pub async fn read_student(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    
        // // Retrieves the database binding DB
        // let d1 = ctx.env.d1("DB")?;

        // // A SQL query to fetch all columns from the student table
        // let query = "SELECT * FROM student";

        // // using a first this is only access a first row in the database
        // let statement = d1.prepare(query).first::<StudentData>(None).await?;

        // //using all function access a all rows in the database
        // // let statement = d1.prepare(query).all::<StudentData>(None).await?;

        // if let Some(student) = statement {
        //     Response::from_json(&student)
        // } else {
        //     Response::error("Student data not found", 404)
        // }


        if let Some(student_id) = ctx.param("id"){
            let d1 = ctx.env.d1("DB")?;

            let check_query = "SELECT * FROM student WHERE id = ?";

            let statement = d1.prepare(check_query).bind(&[student_id.into()])?;
            let check_result = statement.run().await?;

            let result:Vec<Value> = check_result.results()?;

            if result.is_empty(){
                return Response::from_json(&json!({
                    "status": "error",
                    "message": "Student Not Found"
                }));
            }else {
                let read_query = "SELECT * FROM student WHERE id = ?";
                let read_statement = d1.prepare(read_query).bind(&[student_id.into()])?;
                // let read_result = read_statement.first(None).await?;
                let read_result = read_statement.first::<StudentData>(None).await?;
                // let read_result = read_statement.all().await?;

                if let Some(student) = read_result {
                        return Response::from_json(&student);
                } else {
                    return Response::error("Student data not found", 404);
                }
                // Response::ok("Read data successfully")

            }
   
        }else {
            return Response::from_json(&json!({
                "status":"error",
                "message": "User Id Not Found"
             }));
        }
    }
