

use serde_json::json;
use crate::wasm_bindgen::JsValue;


use serde::{Deserialize, Serialize};
use worker::{console_log, Request, Response, Result, RouteContext};


#[derive(Serialize,Deserialize,Debug)]
pub struct UpdateStudent{
    id: Option<i32>,
    name: Option<String>,
    age: Option<i32>
}

pub async fn update_student(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {

    // Response::ok("Update student data successfully")

    if let Some(student_id) = ctx.param("id"){
        let d1 = ctx.env.d1("DB")?; //binding the database d1
        let check_query = "SELECT * FROM student where id = ?";

        // praper and run the database query
        let statement = d1.prepare(check_query).bind(&[student_id.into()])?;
        let check_result = statement.run().await?;

        console_log!("student id {:?}",student_id);

        // console_log!("SQL Query: {}", update_query);

        let results: Vec<serde_json::Value> = check_result.results()?;

        if results.is_empty(){
            Response::from_json(&json!({
                "status": "error",
                "message ": "student  not found"
            }))
        }else {
            let update_data: UpdateStudent = req.json().await?;
            console_log!("Received update data {:?}", update_data);

            // let update_query = "UPDATE student SET name = ?, age = ? WHERE id = ?";
            

            // let update_statement = d1.prepare(update_query).bind(&[
            //     update_data.name.clone().unwrap_or_default().into(),
            //     update_data.age.unwrap_or(0).into(),
            //     student_id.into(),
            // ])?;
            // let update_result = update_statement.run().await?; 
           
           let mut  update_query = String::from("UPDATE student SET");
           let mut  params:Vec<JsValue> = Vec::new();

           if let Some(name) = update_data.name{
            update_query.push_str(" name = ?, ");
            params.push(JsValue::from(name));
           }
           if let Some(age) = update_data.age{
            update_query.push_str(" age = ?, ");
            params.push(JsValue::from(age));
           }

           update_query.truncate(update_query.len() - 2); // Remove trailing comma
           update_query.push_str(" WHERE id = ?");
           params.push(JsValue::from(student_id)); // Correct for i32

           let upate_statement = d1.prepare(&update_query).bind(&params);
           let update_result = upate_statement?.run().await?;

            if update_result.success(){
                return Response::from_json(&json!({
                    "status": "success",
                    "message": "Student updated successfully",
                }));
            } else {
                return Response::from_json(&json!({
                    "status": "error",
                    "message": "Failed to update student",
                }));
            }
        }
    }else {
        
        // Add an `else` block to handle missing `id` parameter
        Response::from_json(&json!({
            "status": "error",
            "message": "student ID is missing"
        }))
    }

    


}