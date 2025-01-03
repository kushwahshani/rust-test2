use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::{console_log, Request, Response, Result, RouteContext};

#[derive(Serialize,Deserialize,Debug)]
pub struct DeleteData{
    id:i32
}

pub async fn delete_student( _req:Request, ctx: RouteContext<()>) -> Result<Response> {
   

    if let Some(student_id) = ctx.param("id"){
        let d1 = ctx.env.d1("DB")?;
        let check_query = "SELECT * FROM student WHERE id = ?";

        let statement = d1.prepare(check_query).bind(&[student_id.into()])?;
        let check_result = statement.run().await?;

        console_log!("student id {:?}", student_id);

        let result: Vec<serde_json::Value> = check_result.results()?;

        if result.is_empty(){
            return Response::from_json(&json!({
                "status" : "error",
                "message" : "Studnet Not Found"
            }));
        }else {
            // let delete_data: DeleteData = req.json().await?;

            let delete_query = "DELETE FROM student WHERE id = ?";
            console_log!("student delete query {:?}", delete_query);


            let delete_statement = d1.prepare(delete_query).bind(&[student_id.into()])?;
            let delete_result = delete_statement.run().await?;

            if delete_result.success(){
                return Response::from_json(&json!({
                    "status":"success",
                    "message":"Studnet Data Delete Successfully"
                }));
            }else {
                return Response::from_json(&json!({
                    "status":"error",
                    "message":"Student Data Delete Failed "
                }));
            }
            // Response::ok("delete student data successfully")
        }
    }else {
         return Response::from_json(&json!({
            "status":"error",
            "message": "User Id Not Found"
         }));
    }
}